use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
    adapters::outbound::{db::users::UsersRepoImpl, rest::dtos::users::UserOutputDto},
    domain::repositories::users::UsersRepo,
    shared::errors::{Rfc7807Problem, StandardOutput, StandardResponse},
};

pub struct GetUserByEmailUseCase {
    users_repo: UsersRepoImpl,
    db: Arc<DatabaseConnection>,
}

impl GetUserByEmailUseCase {
    pub fn new(users_repo: UsersRepoImpl, db: Arc<DatabaseConnection>) -> Self {
        Self { users_repo, db }
    }

    pub async fn execute(&self, email: &str) -> StandardOutput<UserOutputDto> {
        let user = self
            .users_repo
            .find_by_email(email, &*self.db)
            .await?
            .data
            .ok_or_else(|| {
                Box::new(Rfc7807Problem::not_found(
                    Some("Usuario no encontrado"),
                    Some("No existe un usuario con el email proporcionado"),
                ))
            })?;

        Ok(StandardResponse::simple(user.into(), None))
    }
}
