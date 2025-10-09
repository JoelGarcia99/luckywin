use std::sync::Arc;

use axum::http::StatusCode;
use sea_orm::{ActiveValue::Set, ConnectionTrait};

use crate::{
    adapters::{
        inbounds::rest::dtos::client::ClientsInputDto,
        outbound::rest::dtos::client::ClientOutputDto,
    },
    domain::{entities::users, enums::roles::Roles, repositories::clients_repository::ClientsRepo},
    shared::errors::StandardOutput,
};

/// Purchases a new set of lottery tickets, each with length 5. Max 5 tickets, min 1
pub struct UpsertClientUseCase<R: ClientsRepo, C: ConnectionTrait> {
    repo: R,
    db: Arc<C>,
}

impl<R: ClientsRepo, C: ConnectionTrait> UpsertClientUseCase<R, C> {
    pub fn new(repo: R, db: Arc<C>) -> Self {
        Self { repo, db }
    }

    pub async fn execute(&self, client: &ClientsInputDto) -> StandardOutput<ClientOutputDto> {
        let active_model = users::ActiveModel {
            email: Set(client.email.clone()),
            first_name: Set(client.first_name.clone()),
            last_name: Set(client.last_name.clone()),
            cellphone: Set(client.cellphone.clone()),
            location: Set(client.location.clone()),
            role: Set(Roles::Client.to_string()),
            ..Default::default()
        };

        // determining if the user already exists
        match self
            .repo
            .get_client_by_email(&client.email, &*self.db)
            .await
        {
            Ok(client) => {
                self.repo
                    .update_client(&client.data.id, &active_model, &*self.db)
                    .await
            }
            Err(e) => {
                if e.status() != StatusCode::NOT_FOUND {
                    return Err(e);
                }

                self.repo.create_client(&active_model, &*self.db).await
            }
        }
    }
}
