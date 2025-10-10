use std::sync::Arc;

use sea_orm::ConnectionTrait;
use uuid::Uuid;

use crate::{
    adapters::inbounds::rest::dtos::auth::LogoutOutputDto,
    domain::repositories::sessions::SessionsRepo,
    shared::errors::{StandardOutput, StandardResponse},
};

pub struct LogoutUseCase<SR: SessionsRepo, C: ConnectionTrait> {
    sessions_repo: SR,
    db: Arc<C>,
}

impl<SR: SessionsRepo + Clone, C: ConnectionTrait> LogoutUseCase<SR, C> {
    pub fn new(sessions_repo: SR, db: Arc<C>) -> Self {
        Self { sessions_repo, db }
    }

    pub async fn execute(
        &self,
        user_id: &Uuid,
        device_fingerprint: &str,
    ) -> StandardOutput<LogoutOutputDto> {
        tracing::debug!(
            "Logging out user ID: {}, device fingerprint: {}",
            user_id,
            device_fingerprint
        );

        self.sessions_repo
            .soft_delete_by_user_and_device(user_id, device_fingerprint, &*self.db)
            .await?;

        Ok(StandardResponse::simple(
            LogoutOutputDto {
                message: "Sesión cerrada correctamente".to_string(),
            },
            Some("Sesión cerrada correctamente"),
        ))
    }
}
