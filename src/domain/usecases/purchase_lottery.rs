use std::sync::Arc;

use chrono::FixedOffset;
use sea_orm::{ActiveValue::Set, DatabaseConnection, TransactionTrait};
use uuid::Uuid;

use crate::{
    adapters::{
        inbounds::rest::dtos::{client::ClientsInputDto, lottery::LotteriesPurchaseDto},
        outbound::{db::error_parsing::process_db_err, rest::dtos::lottery::LotteryOutputDto},
    },
    domain::{
        entities::lotteries,
        enums::lottery_status::LotteryStatus,
        repositories::{clients::ClientsRepo, games::GamesRepo, lotteries::LotteriesRepo},
        usecases::{
            get_last_active_game::GetLastActiveGameUseCase, upsert_client::UpsertClientUseCase,
        },
    },
    shared::{
        errors::{Rfc7807Problem, StandardOutput},
        utils::bitboards,
    },
};

/// Purchases a new set of lottery tickets, each with length 5. Max 5 tickets, min 1
pub struct PurchaseLotteryUseCase<R: LotteriesRepo, R2: GamesRepo, R3: ClientsRepo> {
    lottery_repo: R,
    games_repo: R2,
    clients_repo: R3,
    db: Arc<DatabaseConnection>,
}

impl<R: LotteriesRepo, R2: GamesRepo, R3: ClientsRepo + Clone> PurchaseLotteryUseCase<R, R2, R3> {
    pub fn new(
        lottery_repo: R,
        games_repo: R2,
        clients_repo: R3,
        db: Arc<DatabaseConnection>,
    ) -> Self {
        Self {
            lottery_repo,
            games_repo,
            clients_repo,
            db,
        }
    }

    pub async fn execute(
        &self,
        lotteries: LotteriesPurchaseDto,
    ) -> StandardOutput<Vec<LotteryOutputDto>> {
        // transactions in order to create lotteries and upsert the client
        let txn = self.db.begin().await.unwrap();

        let last_active_game_usecase = GetLastActiveGameUseCase::new(self.games_repo, &txn);
        let upsert_client_by_email_usecase = UpsertClientUseCase::new(self.clients_repo, &txn);

        // getting the last active game if any
        let active_game_promise = last_active_game_usecase.execute();

        // getting the client via upsert
        let clients_dto = ClientsInputDto {
            first_name: lotteries.client_name.clone(),
            last_name: lotteries.client_last_name.clone(),
            email: lotteries.client_email.clone(),
            cellphone: lotteries.client_cellphone.clone(),
            location: lotteries.location.clone(),
        };

        let client_promise = upsert_client_by_email_usecase.execute(&clients_dto);

        let (game, client) = match tokio::join!(active_game_promise, client_promise) {
            (Ok(game), Ok(client)) => (game.data, client.data),
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        };

        let length = lotteries.tickets.len();

        // the number of tickets must be between 1 and 5
        if !(1..=(game.max_tickets_per_lottery as usize)).contains(&length) {
            return Err(Box::new(Rfc7807Problem::simple_bad_request(
                Some("Error al comprar sus tickets"),
                Some("Debe comprar entre 1 y 5 tockets"),
                None,
            )));
        }

        // all the tickets must have 5 numbers
        for ticket in &lotteries.tickets {
            if ticket.numbers.len() != 5 { // TODO: parameterize numbers in ticket
                return Err(Box::new(Rfc7807Problem::simple_bad_request(
                    Some("Error al comprar sus tickets"),
                    Some("Todos los tickets deben tener 5 números"),
                    None,
                )));
            }
        }

        // creating proper active models
        let mut active_models = Vec::new();

        for ticket in lotteries.tickets {
            active_models.push(lotteries::ActiveModel {
                id: Set(Uuid::new_v4()),
                numbers: Set(bitboards::parse_vec_into_bitboard(ticket.numbers)),
                id_game: Set(game.id),
                email: Set(Some(client.email.clone())),
                client_name: Set(format!(
                    "{} {}",
                    client.name,
                    client.last_name.clone().unwrap_or_default()
                )),
                total_paid: Set(game.ticket_price),
                location: Set(lotteries.location.clone()),
                status: Set(LotteryStatus::Unpaid.to_string()),
                cellphone: Set(Some(lotteries.client_cellphone.clone())),
                id_client: Set(client.id),
                created_at: Set(chrono::DateTime::<FixedOffset>::from(chrono::Utc::now())),
                updated_at: Set(chrono::DateTime::<FixedOffset>::from(chrono::Utc::now())),
                ..Default::default()
            });
        }

        let lotteries = self.lottery_repo.batch_create(active_models, &txn).await?;

        match txn.commit().await {
            Ok(_) => Ok(lotteries),
            Err(e) => Err(process_db_err(&e)),
        }
    }
}
