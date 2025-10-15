use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    adapters::outbound::{
        db::{games::GamesRepoImpl, lotteries::LotteriesRepoImpl, users::UsersRepoImpl},
        rest::dtos::purchases::TicketOutputDto,
    },
    domain::{
        repositories::lotteries::LotteriesRepo,
        usecases::{
            get_last_active_game::GetLastActiveGameUseCase,
            get_user_by_cellphone::GetUserByCellphoneUseCase,
        },
    },
    shared::errors::StandardOutput,
};

pub struct GetTicketsForUser<'a> {
    users_repo: UsersRepoImpl,
    games_repo: GamesRepoImpl,
    lotteries_repo: LotteriesRepoImpl,
    cellphone: &'a str,
    db: Arc<DatabaseConnection>,
}

impl<'a> GetTicketsForUser<'a> {
    pub fn new(
        users_repo: UsersRepoImpl,
        games_repo: GamesRepoImpl,
        lotteries_repo: LotteriesRepoImpl,
        cellphone: &'a str,
        db: Arc<DatabaseConnection>,
    ) -> Self {
        Self {
            users_repo,
            games_repo,
            lotteries_repo,
            cellphone,
            db,
        }
    }

    pub async fn execute(&self) -> StandardOutput<Vec<TicketOutputDto>> {
        // getting target user
        let user_by_phone_usecase =
            GetUserByCellphoneUseCase::new(self.users_repo, self.db.clone());

        let user_id = user_by_phone_usecase.execute(self.cellphone).await?.data.id;

        // getting target gaem
        let last_active_game_usecase = GetLastActiveGameUseCase::new(self.games_repo, &*self.db);
        let game_id = last_active_game_usecase.execute().await?.data.id;

        self.lotteries_repo
            .find_tickets_by_user(&user_id, &game_id, &*self.db)
            .await
    }
}
