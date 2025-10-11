use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
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

        Ok(StandardResponse::created(inserted.into()))
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
            .map(|game| game.into())
            .collect::<Vec<GameOutputDto>>();

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
            game.into(),
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
            game.into(),
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
                .map(|game| game.into())
                .collect::<Vec<GameOutputDto>>(),
            None,
        ))
    }

    async fn close_game<C>(&self, game_id: &Uuid, db: &C) -> StandardOutput<GameOutputDto>
    where
        C: ConnectionTrait,
    {
        // Find the game by ID, excluding soft-deleted records
        let game = Entity::find_by_id(*game_id)
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?
            .ok_or_else(|| {
                Rfc7807Problem::not_found(
                    Some("Juego no encontrado"),
                    Some("El juego especificado no existe o ha sido eliminado"),
                )
            })?;

        // Check if game is already finished
        if game.finished_at.is_some() {
            return Err(Box::new(Rfc7807Problem::simple_bad_request(
                Some("El juego ya está cerrado"),
                Some("Este juego ya fue cerrado anteriormente"),
                Some("GAME_ALREADY_CLOSED"),
            )));
        }

        // Update the finished_at timestamp
        let mut active_model: games::ActiveModel = game.into();
        active_model.finished_at = Set(Some(chrono::Utc::now().into()));
        active_model.updated_at = Set(chrono::Utc::now().into());

        let updated_game = active_model.update(db).await.map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(
            updated_game.into(),
            None,
        ))
    }
}
