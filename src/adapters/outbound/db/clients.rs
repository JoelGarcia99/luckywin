use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
    adapters::outbound::{db::error_parsing::process_db_err, rest::dtos::client::ClientOutputDto},
    domain::{
        entities::users::{self, Column, Entity},
        repositories::clients::ClientsRepo,
    },
    shared::errors::{PaginatorResponse, Rfc7807Problem, StandardOutput, StandardResponse},
};

#[derive(Clone, Copy)]
pub struct ClientsRepoImpl;

#[async_trait]
impl ClientsRepo for ClientsRepoImpl {
    async fn create_client<C>(
        &self,
        client: &users::ActiveModel,
        db: &C,
    ) -> StandardOutput<ClientOutputDto>
    where
        C: ConnectionTrait,
    {
        let inserted = client
            .clone()
            .insert(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::created(ClientOutputDto {
            id: inserted.id,
            name: inserted.first_name,
            last_name: inserted.last_name,
            email: inserted.email,
            cellphone: inserted.cellphone,
            location: inserted.location,
        }))
    }

    async fn update_client<C>(
        &self,
        client_id: &Uuid,
        client: &users::ActiveModel,
        db: &C,
    ) -> StandardOutput<ClientOutputDto>
    where
        C: ConnectionTrait,
    {
        // Find the existing client
        let existing = Entity::find_by_id(*client_id)
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?
            .ok_or_else(|| {
                Rfc7807Problem::not_found(
                    Some("Cliente no existe"),
                    Some("El cliente solicitado no existe"),
                )
            })?;

        // Convert to active model and update fields
        let mut active_model: users::ActiveModel = existing.into();

        // Update only the provided fields
        if let Set(first_name) = &client.first_name {
            active_model.first_name = Set(first_name.clone());
        }
        if let Set(last_name) = &client.last_name {
            active_model.last_name = Set(last_name.clone());
        }
        if let Set(cellphone) = &client.cellphone {
            active_model.cellphone = Set(cellphone.clone());
        }
        if let Set(location) = &client.location {
            active_model.location = Set(location.clone());
        }

        let updated = active_model
            .update(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(
            ClientOutputDto {
                id: updated.id,
                name: updated.first_name,
                last_name: updated.last_name,
                email: updated.email,
                cellphone: updated.cellphone,
                location: updated.location,
            },
            None,
        ))
    }

    async fn get_client_by_id<C>(&self, id: &str, db: &C) -> StandardOutput<ClientOutputDto>
    where
        C: ConnectionTrait,
    {
        let uuid = Uuid::parse_str(id).map_err(|_| {
            Rfc7807Problem::simple_bad_request(
                Some("Invalid UUID"),
                Some("The provided ID is not a valid UUID"),
                Some("INVALID_UUID"),
            )
        })?;

        let client = Entity::find_by_id(uuid)
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?
            .ok_or_else(|| {
                Rfc7807Problem::not_found(
                    Some("Cliente no encontrado"),
                    Some("El cliente solicitado no existe"),
                )
            })?;

        Ok(StandardResponse::simple(
            ClientOutputDto {
                id: client.id,
                name: client.first_name,
                last_name: client.last_name,
                email: client.email,
                cellphone: client.cellphone,
                location: client.location,
            },
            None,
        ))
    }

    async fn get_client_by_cellphone<C>(
        &self,
        cellphone: &str,
        db: &C,
    ) -> StandardOutput<ClientOutputDto>
    where
        C: ConnectionTrait,
    {
        let client = Entity::find()
            .filter(Column::Cellphone.eq(cellphone))
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?
            .ok_or_else(|| {
                Rfc7807Problem::not_found(
                    Some("Cliente no encontrado"),
                    Some("El cliente solicitado no existe"),
                )
            })?;

        Ok(StandardResponse::simple(
            ClientOutputDto {
                id: client.id,
                name: client.first_name,
                last_name: client.last_name,
                email: client.email,
                cellphone: client.cellphone,
                location: client.location,
            },
            None,
        ))
    }

    /// List a paginated result of all non soft deleted clients
    async fn list_clients<C>(
        &self,
        db: &C,
        page: u64,
        per_page: u64,
    ) -> StandardOutput<Vec<ClientOutputDto>>
    where
        C: ConnectionTrait,
    {
        let clients_paginator = Entity::find()
            .filter(Column::DeletedAt.is_null())
            .order_by_desc(Column::CreatedAt)
            .paginate(db, per_page);

        let total_items_promise = clients_paginator.num_items();
        let data_promise = clients_paginator.fetch_page(page);

        let (total_records, clients) = match tokio::join!(total_items_promise, data_promise) {
            (Ok(pages), Ok(data)) => (pages, data),
            (Err(e), _) => return Err(process_db_err(&e)),
            (_, Err(e)) => return Err(process_db_err(&e)),
        };

        let dtos = clients
            .into_iter()
            .map(|client| ClientOutputDto {
                id: client.id,
                name: client.first_name,
                last_name: client.last_name,
                email: client.email,
                cellphone: client.cellphone,
                location: client.location,
            })
            .collect();

        let paginator = PaginatorResponse::new(total_records, page, per_page);
        Ok(StandardResponse::with_paginator(dtos, paginator, None))
    }
}
