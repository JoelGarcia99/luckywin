use std::sync::Arc;

use chrono::Utc;
use sea_orm::{DatabaseConnection, Set};
use serde_valid::Validate;
use uuid::Uuid;

use crate::{
    adapters::{
        inbounds::rest::dtos::user::UpdateUserDto,
        outbound::{
            db::users::UsersRepoImpl,
            rest::dtos::users::UserOutputDto,
        },
    },
    domain::{entities::users, repositories::users::UsersRepo},
    shared::errors::{Rfc7807Problem, StandardOutput, StandardResponse},
};

pub struct UpdateCurrentUserUseCase {
    users_repo: UsersRepoImpl,
    db: Arc<DatabaseConnection>,
}

impl UpdateCurrentUserUseCase {
    pub fn new(users_repo: UsersRepoImpl, db: Arc<DatabaseConnection>) -> Self {
        Self { users_repo, db }
    }

    pub async fn execute(
        &self,
        user_id: &Uuid,
        dto: &UpdateUserDto,
    ) -> StandardOutput<UserOutputDto> {
        dto.validate()
            .map_err(|e| Rfc7807Problem::from_validator_err(&e.to_string()))?;

        let user = self
            .users_repo
            .find_by_id(user_id, &*self.db)
            .await?
            .data
            .ok_or_else(|| {
                Box::new(Rfc7807Problem::not_found(
                    Some("Usuario no encontrado"),
                    None,
                ))
            })?;

        let mut active_model = users::ActiveModel{
            id: Set(user.id),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        };


        if let Some(first_name) = &dto.first_name {
            active_model.first_name = Set(first_name.clone());
        }

        if let Some(last_name) = &dto.last_name {
            active_model.last_name = Set(Some(last_name.clone()));
        }

        if let Some(location) = &dto.location {
            active_model.location = Set(Some(location.clone()));
        }

        if let Some(cellphone) = &dto.cellphone {
            active_model.cellphone = Set(cellphone.clone());
        }

        if let Some(email) = &dto.email {
            active_model.email = Set(email.clone());
        }

        let updated_user = self
            .users_repo
            .update_user(&active_model, &*self.db)
            .await?
            .data;

        Ok(StandardResponse::simple(
            UserOutputDto {
                id: updated_user.id,
                first_name: updated_user.first_name,
                last_name: updated_user.last_name,
                location: updated_user.location,
                cellphone: updated_user.cellphone,
                email: updated_user.email,
                role: updated_user.role,
                created_at: updated_user.created_at.into(),
                updated_at: updated_user.updated_at.into(),
            },
            Some("Usuario actualizado correctamente"),
        ))
    }
}
