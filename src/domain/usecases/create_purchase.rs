use std::{collections::HashSet, sync::Arc};

use chrono::FixedOffset;
use sea_orm::{ActiveValue::Set, DatabaseConnection, TransactionTrait, prelude::Decimal};
use uuid::Uuid;

use crate::{
    adapters::{
        inbounds::rest::dtos::{client::ClientsInputDto, purchase::CreatePurchaseDto},
        outbound::{db::error_parsing::process_db_err, rest::dtos::purchases::PurchaseOutputDto},
    },
    domain::{
        entities::lotteries,
        repositories::{
            clients::ClientsRepo, games::GamesRepo, lotteries::LotteriesRepo,
            purchases::PurchasesRepo,
        },
        usecases::{
            get_last_active_game::GetLastActiveGameUseCase, upsert_client::UpsertClientUseCase,
        },
    },
    shared::{
        errors::{Rfc7807Problem, StandardOutput},
        utils::bitboards,
    },
};

/// Creates a new purchase with lottery tickets
pub struct CreatePurchaseUseCase<
    R: LotteriesRepo,
    R2: GamesRepo,
    R3: ClientsRepo,
    R4: PurchasesRepo,
> {
    lottery_repo: R,
    games_repo: R2,
    clients_repo: R3,
    purchases_repo: R4,
    db: Arc<DatabaseConnection>,
}

impl<R: LotteriesRepo, R2: GamesRepo, R3: ClientsRepo + Clone, R4: PurchasesRepo>
    CreatePurchaseUseCase<R, R2, R3, R4>
{
    pub fn new(
        lottery_repo: R,
        games_repo: R2,
        clients_repo: R3,
        purchases_repo: R4,
        db: Arc<DatabaseConnection>,
    ) -> Self {
        Self {
            lottery_repo,
            games_repo,
            clients_repo,
            purchases_repo,
            db,
        }
    }

    pub async fn execute(
        &self,
        purchase_dto: CreatePurchaseDto,
    ) -> StandardOutput<PurchaseOutputDto> {
        // Begin transaction to create purchase and lottery tickets atomically
        let txn = self.db.begin().await.unwrap();

        let last_active_game_usecase = GetLastActiveGameUseCase::new(self.games_repo, &txn);
        let upsert_client_by_email_usecase = UpsertClientUseCase::new(self.clients_repo, &txn);

        // Get the last active game if any
        let active_game_promise = last_active_game_usecase.execute();

        // Get or create the client
        let clients_dto = ClientsInputDto {
            first_name: purchase_dto.client_name.clone(),
            last_name: purchase_dto.client_last_name.clone(),
            email: purchase_dto.client_email.clone(),
            cellphone: purchase_dto.client_cellphone.clone(),
            location: purchase_dto.location.clone(),
        };

        let client_promise = upsert_client_by_email_usecase.execute(&clients_dto);

        let (game, client) = match tokio::join!(active_game_promise, client_promise) {
            (Ok(game), Ok(client)) => (game.data, client.data),
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        };

        let num_tickets = purchase_dto.tickets.len();

        // The number of tickets must be between 1 and max_tickets_per_lottery
        if !(1..=(game.max_tickets_per_lottery as usize)).contains(&num_tickets) {
            return Err(Box::new(Rfc7807Problem::simple_bad_request(
                Some("Error al comprar sus tickets"),
                Some(&format!(
                    "Debe comprar entre 1 y {} tickets",
                    game.max_tickets_per_lottery
                )),
                None,
            )));
        }

        // Memoized tickets to avoid duplicates
        let mut memoized_tickets: HashSet<Decimal> = HashSet::new();

        // All tickets must have 5 numbers
        for ticket in &purchase_dto.tickets {
            if ticket.numbers.len() != 5 {
                // TODO: parameterize numbers in ticket
                return Err(Box::new(Rfc7807Problem::simple_bad_request(
                    Some("Error al comprar sus tickets"),
                    Some("Todos los tickets deben tener 5 números"),
                    None,
                )));
            }

            // All numbers must be between 1 and 75
            for &number in &ticket.numbers {
                if !(1..=75).contains(&number) {
                    return Err(Box::new(Rfc7807Problem::simple_bad_request(
                        Some("Error al comprar sus tickets"),
                        Some("Todos los números deben estar entre 1 y 75"),
                        None,
                    )));
                }
            }

            // Transform into bitboards
            let binary = bitboards::parse_vec_into_bitboard(ticket.numbers.clone());

            // Check for duplicates
            if !memoized_tickets.insert(binary) {
                return Err(Box::new(Rfc7807Problem::simple_bad_request(
                    Some("Error al comprar sus tickets"),
                    Some("No puede comprar tickets duplicados"),
                    None,
                )));
            }
        }

        // Each purchase has the same ticket_price regardless of number of tickets
        let total_paid = game.ticket_price;

        // Create the purchase first (serial will be auto-incremented by the database)
        let mut purchase = self
            .purchases_repo
            .create(
                &game.id,
                &client.id,
                &format!(
                    "{} {}",
                    client.name,
                    client.last_name.clone().unwrap_or_default()
                ),
                client.email.as_deref(),
                Some(&purchase_dto.client_cellphone),
                purchase_dto.location.as_deref(),
                total_paid,
                &txn,
            )
            .await?
            .data;

        // Create lottery tickets linked to this purchase
        let mut lottery_active_models = Vec::new();

        for ticket in purchase_dto.tickets {
            lottery_active_models.push(lotteries::ActiveModel {
                id: Set(Uuid::new_v4()),
                numbers: Set(bitboards::parse_vec_into_bitboard(ticket.numbers)),
                id_game: Set(game.id),
                id_purchase: Set(Some(purchase.id)),
                created_at: Set(chrono::DateTime::<FixedOffset>::from(chrono::Utc::now())),
                updated_at: Set(chrono::DateTime::<FixedOffset>::from(chrono::Utc::now())),
                ..Default::default()
            });
        }

        let created_tickets = self
            .lottery_repo
            .batch_create(lottery_active_models, &txn)
            .await?
            .data;

        // Add tickets to purchase response
        purchase.tickets = created_tickets
            .into_iter()
            .map(
                |t| crate::adapters::outbound::rest::dtos::purchases::TicketOutputDto {
                    id: t.id,
                    serial: t.serial,
                    numbers: t.numbers,
                    created_at: t.created_at,
                    updated_at: t.created_at, // Use created_at since updated_at doesn't exist in LotteryOutputDto
                },
            )
            .collect();

        match txn.commit().await {
            Ok(_) => Ok(crate::shared::errors::StandardResponse::simple(
                purchase, None,
            )),
            Err(e) => Err(process_db_err(&e)),
        }
    }
}
