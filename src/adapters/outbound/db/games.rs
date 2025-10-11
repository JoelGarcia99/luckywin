use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use uuid::Uuid;

use crate::{
    adapters::outbound::{db::error_parsing::process_db_err, rest::dtos::game::GameOutputDto},
    domain::{
        entities::games::{self, Column, Entity},
        repositories::games::GamesRepo,
    },
    shared::errors::{PaginatorResponse, Rfc7807Problem, StandardOutput, StandardResponse},
};

#[derive(Clone, Copy)]
pub struct GamesRepoImpl;

#[async_trait]
impl GamesRepo for GamesRepoImpl {
    async fn create<C>(&self, game: &games::ActiveModel, db: &C) -> StandardOutput<GameOutputDto>
    where
        C: ConnectionTrait,
    {
        let inserted = game
            .clone()
            .insert(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::created(GameOutputDto {
            id: inserted.id,
            title: inserted.title,
            banner_url: inserted.banner_url,
            ticket_price: inserted.ticket_price,
            finished_at: inserted.finished_at.map(|dt| dt.into()),
            created_at: inserted.created_at.into(),
            updated_at: inserted.updated_at.into(),
        }))
    }

    async fn list_paginated<C>(
        &self,
        page: u64,
        per_page: u64,
        db: &C,
    ) -> StandardOutput<Vec<GameOutputDto>>
    where
        C: ConnectionTrait,
    {
        let paginator = Entity::find()
            .filter(Column::DeletedAt.is_null())
            .order_by_desc(Column::CreatedAt)
            .paginate(db, per_page);

        let total_records_promise = paginator.num_items();
        let data_promise = paginator.fetch_page(page);

        let (total_records, games) = match tokio::join!(total_records_promise, data_promise) {
            (Ok(pages), Ok(data)) => (pages, data),
            (Err(e), _) => return Err(process_db_err(&e)),
            (_, Err(e)) => return Err(process_db_err(&e)),
        };

        let dtos = games
            .into_iter()
            .map(|game| GameOutputDto {
                id: game.id,
                title: game.title,
                banner_url: game.banner_url,
                ticket_price: game.ticket_price,
                finished_at: game.finished_at.map(|dt| dt.into()),
                created_at: game.created_at.into(),
                updated_at: game.updated_at.into(),
            })
            .collect();

        let paginator = PaginatorResponse::new(total_records, page, per_page);

        Ok(StandardResponse::with_paginator(dtos, paginator, None))
    }

    async fn get_by_id<C>(&self, id: &Uuid, db: &C) -> StandardOutput<GameOutputDto>
    where
        C: ConnectionTrait,
    {
        let game = Entity::find_by_id(*id)
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?
            .ok_or_else(|| {
                Rfc7807Problem::not_found(
                    Some("Juego no encontrado"),
                    Some("El juego solicitado no existe"),
                )
            })?;

        Ok(StandardResponse::simple(
            GameOutputDto {
                id: game.id,
                title: game.title,
                banner_url: game.banner_url,
                ticket_price: game.ticket_price,
                finished_at: game.finished_at.map(|dt| dt.into()),
                created_at: game.created_at.into(),
                updated_at: game.updated_at.into(),
            },
            None,
        ))
    }

    async fn get_last_active<C>(&self, db: &C) -> StandardOutput<GameOutputDto>
    where
        C: ConnectionTrait,
    {
        let game = Entity::find()
            .filter(Column::FinishedAt.is_null())
            .filter(Column::DeletedAt.is_null())
            .order_by_desc(Column::CreatedAt)
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?
            .ok_or_else(|| {
                Rfc7807Problem::not_found(
                    Some("No hay juegos activos"),
                    Some("No hay ningún juego activo por el momento, por favor intente más tarde"),
                )
            })?;

        Ok(StandardResponse::simple(
            GameOutputDto {
                id: game.id,
                title: game.title,
                banner_url: game.banner_url,
                ticket_price: game.ticket_price,
                finished_at: game.finished_at.map(|dt| dt.into()),
                created_at: game.created_at.into(),
                updated_at: game.updated_at.into(),
            },
            None,
        ))
    }

    ///
    async fn get_batch_by_ids<C>(
        &self,
        db: &C,
        ids: &Vec<Uuid>,
    ) -> StandardOutput<Vec<GameOutputDto>>
    where
        C: ConnectionTrait,
    {
        let game = Entity::find()
            .filter(Column::FinishedAt.is_null())
            .filter(Column::DeletedAt.is_null())
            .filter(Column::Id.is_in(ids.clone().into_iter().map(|id| id).collect::<Vec<Uuid>>()))
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(
            game.into_iter()
                .map(|game| GameOutputDto {
                    id: game.id,
                    title: game.title,
                    banner_url: game.banner_url,
                    ticket_price: game.ticket_price,
                    finished_at: game.finished_at.map(|dt| dt.into()),
                    created_at: game.created_at.into(),
                    updated_at: game.updated_at.into(),
                })
                .collect(),
            None,
        ))
    }
}
