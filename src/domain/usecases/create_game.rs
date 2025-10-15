use std::sync::Arc;

use axum::http::StatusCode;
use chrono::Utc;
use sea_orm::{ActiveValue::Set, ConnectionTrait};
use uuid::Uuid;

use crate::{
    adapters::{
        inbounds::rest::dtos::game::CreateGameDto, outbound::rest::dtos::game::GameOutputDto,
    },
    domain::{
        entities::games, repositories::games::GamesRepo,
        usecases::get_last_active_game::GetLastActiveGameUseCase,
    },
    shared::errors::{Rfc7807Problem, StandardOutput},
};

/// Creates a new game/lottery
pub struct CreateGameUseCase<R: GamesRepo, C: ConnectionTrait> {
    repo: R,
    db: Arc<C>,
}

impl<R: GamesRepo + Clone, C: ConnectionTrait> CreateGameUseCase<R, C> {
    pub fn new(repo: R, db: Arc<C>) -> Self {
        Self { repo, db }
    }

    pub async fn execute(&self, game_dto: &CreateGameDto) -> StandardOutput<GameOutputDto> {
        let last_active_use_case = GetLastActiveGameUseCase::new(self.repo, &*self.db);

        // getting last active game first to see if we can enable a new one
        match last_active_use_case.execute().await {
            Ok(game) => {
                return Err(Box::new(Rfc7807Problem::simple_bad_request(
                    Some("Ya hay un juego activo"),
                    Some(&format!(
                        "Cierre el juego {} para poder crear uno nuevo",
                        game.data.title
                    )),
                    None,
                )));
            }
            Err(e) => {
                if e.status() != StatusCode::NOT_FOUND {
                    return Err(e);
                }
            }
        };

        let now = Utc::now();

        let active_model = games::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(game_dto.title.clone()),
            banner_url: Set(game_dto.banner_url.clone()),
            ticket_price: Set(game_dto.ticket_price),
            max_tickets_per_lottery: Set(game_dto.max_tickets_per_lottery),
            closes_at: Set(game_dto.closes_at.map(|dt| dt.into())),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            deleted_at: Set(None),
            ..Default::default()
        };

        self.repo.create(&active_model, &*self.db).await
    }
}
