use std::sync::Arc;

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::outbound::{db::users::UsersRepoImpl, rest::dtos::users::UserOutputDto},
    domain::repositories::users::UsersRepo,
    shared::errors::{Rfc7807Problem, StandardOutput, StandardResponse},
};

pub struct GetCurrentUserUseCase {
    users_repo: UsersRepoImpl,
    db: Arc<DatabaseConnection>,
}

impl GetCurrentUserUseCase {
    pub fn new(users_repo: UsersRepoImpl, db: Arc<DatabaseConnection>) -> Self {
        Self { users_repo, db }
    }

    pub async fn execute(&self, user_id: &Uuid) -> StandardOutput<UserOutputDto> {
        let user = self
            .users_repo
            .find_by_id(user_id, &*self.db)
            .await?
            .data
            .ok_or_else(|| Box::new(Rfc7807Problem::not_found(Some("User not found"), None)))?;

        Ok(StandardResponse::simple(user.into(), None))
    }
}
