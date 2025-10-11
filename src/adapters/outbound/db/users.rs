use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    adapters::outbound::db::error_parsing::process_db_err,
    domain::{
        entities::users::{self, Column, Entity},
        repositories::users::UsersRepo,
    },
    shared::errors::{StandardOutput, StandardResponse},
};

#[derive(Clone, Copy)]
pub struct UsersRepoImpl;

#[async_trait]
impl UsersRepo for UsersRepoImpl {
    async fn find_by_email<C>(&self, email: &str, db: &C) -> StandardOutput<Option<users::Model>>
    where
        C: ConnectionTrait,
    {
        let user = Entity::find()
            .filter(Column::Email.eq(email))
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(user, None))
    }

    async fn find_by_cellphone<C>(
        &self,
        cellphone: &str,
        db: &C,
    ) -> StandardOutput<Option<users::Model>>
    where
        C: ConnectionTrait,
    {
        let user = Entity::find()
            .filter(Column::Cellphone.eq(cellphone))
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(user, None))
    }

    async fn find_by_id<C>(&self, id: &Uuid, db: &C) -> StandardOutput<Option<users::Model>>
    where
        C: ConnectionTrait,
    {
        let user = Entity::find_by_id(*id)
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(user, None))
    }

    async fn update_user<C>(
        &self,
        user: &users::ActiveModel,
        db: &C,
    ) -> StandardOutput<users::Model>
    where
        C: ConnectionTrait,
    {
        user.clone()
            .update(db)
            .await
            .map_err(|e| process_db_err(&e))
            .map(|u| StandardResponse::simple(u, None))
    }
}
