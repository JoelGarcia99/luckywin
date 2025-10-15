use axum::http::StatusCode;
use chrono::FixedOffset;
use sea_orm::{ActiveValue::Set, ConnectionTrait};
use uuid::Uuid;

use crate::{
    adapters::{
        inbounds::rest::dtos::client::ClientsInputDto,
        outbound::rest::dtos::client::ClientOutputDto,
    },
    domain::{entities::users, enums::roles::Roles, repositories::clients::ClientsRepo},
    shared::errors::StandardOutput,
};

/// Purchases a new set of lottery tickets, each with length 5. Max 5 tickets, min 1
pub struct UpsertClientUseCase<'a, R: ClientsRepo, C: ConnectionTrait> {
    repo: R,
    db: &'a C,
}

impl<'a, R: ClientsRepo, C: ConnectionTrait> UpsertClientUseCase<'a, R, C> {
    pub fn new(repo: R, db: &'a C) -> Self {
        Self { repo, db }
    }

    pub async fn execute(&self, client: &ClientsInputDto) -> StandardOutput<ClientOutputDto> {
        let mut active_model = users::ActiveModel {
            email: Set(client.email.clone()),
            first_name: Set(client.first_name.clone()),
            last_name: Set(client.last_name.clone()),
            cellphone: Set(client.cellphone.clone()),
            location: Set(client.location.clone()),
            role: Set(Roles::Client.to_string()),
            updated_at: Set(chrono::DateTime::<FixedOffset>::from(chrono::Utc::now())),
            ..Default::default()
        };

        // determining if the user already exists
        match self
            .repo
            .get_client_by_cellphone(&client.cellphone, self.db)
            .await
        {
            Ok(client) => {
                self.repo
                    .update_client(&client.data.id, &active_model, self.db)
                    .await
            }
            Err(e) => {
                if e.status() != StatusCode::NOT_FOUND {
                    return Err(e);
                }

                active_model.id = Set(Uuid::new_v4());
                active_model.created_at =
                    Set(chrono::DateTime::<FixedOffset>::from(chrono::Utc::now()));

                self.repo.create_client(&active_model, self.db).await
            }
        }
    }
}
