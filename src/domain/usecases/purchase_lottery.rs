use std::sync::Arc;

use sea_orm::{ActiveValue::Set, ConnectionTrait};

use crate::{
    adapters::{
        inbounds::rest::dtos::{client::ClientsInputDto, lottery::LotteriesPurchaseDto},
        outbound::rest::dtos::lottery::LotteryOutputDto,
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
pub struct PurchaseLotteryUseCase<
    R: LotteriesRepo,
    R2: GamesRepo,
    R3: ClientsRepo,
    C: ConnectionTrait,
> {
    lottery_repo: R,
    games_repo: R2,
    clients_repo: R3,
    db: Arc<C>,
}

impl<R: LotteriesRepo, R2: GamesRepo, R3: ClientsRepo + Clone, C: ConnectionTrait>
    PurchaseLotteryUseCase<R, R2, R3, C>
{
    pub fn new(lottery_repo: R, games_repo: R2, clients_repo: R3, db: Arc<C>) -> Self {
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
    ) -> StandardOutput<LotteryOutputDto> {
        let last_active_game_usecase =
            GetLastActiveGameUseCase::new(self.games_repo, self.db.clone());
        let client_by_email_usecase = UpsertClientUseCase::new(self.clients_repo, self.db.clone());

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

        let client_promise = client_by_email_usecase.execute(&clients_dto);

        let (game, client) = match tokio::join!(active_game_promise, client_promise) {
            (Ok(game), Ok(client)) => (game.data, client.data),
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        };

        let length = lotteries.tickets.len();

        // the number of tickets must be between 1 and 5
        if !(1..=5).contains(&length) {
            return Err(Rfc7807Problem::simple_bad_request(
                Some("Error al comprar sus tickets"),
                Some("Debe comprar entre 1 y 5 tockets"),
                None,
            ));
        }

        // all the tickets must have 5 numbers
        for ticket in &lotteries.tickets {
            if ticket.numbers.len() != 5 {
                return Err(Rfc7807Problem::simple_bad_request(
                    Some("Error al comprar sus tickets"),
                    Some("Todos los tickets deben tener 5 números"),
                    None,
                ));
            }
        }

        // creating proper active models
        let mut active_models = Vec::new();

        for ticket in lotteries.tickets {
            active_models.push(lotteries::ActiveModel {
                numbers: Set(bitboards::parse_vec_into_bitboard(ticket.numbers)),
                id_game: Set(game.id),
                email: Set(Some(client.email.clone())),
                client_name: Set(format!(
                    "{} {}",
                    client.name,
                    client.last_name.clone().unwrap_or_default()
                )),
                location: Set(lotteries.location.clone()),
                status: Set(LotteryStatus::Unpaid.to_string()),
                cellphone: Set(Some(lotteries.client_cellphone.clone())),
                id_client: Set(client.id),
                ..Default::default()
            });
        }

        self.lottery_repo.create(active_models, &*self.db).await
    }
}
