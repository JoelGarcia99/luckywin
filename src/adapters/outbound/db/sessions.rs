use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{
    adapters::outbound::db::error_parsing::process_db_err,
    domain::{
        entities::sessions::{self, Column, Entity},
        repositories::sessions::SessionsRepo,
    },
    shared::errors::{StandardOutput, StandardResponse},
};

#[derive(Clone, Copy)]
pub struct SessionsRepoImpl;

#[async_trait]
impl SessionsRepo for SessionsRepoImpl {
    async fn create<C>(
        &self,
        session: &sessions::ActiveModel,
        db: &C,
    ) -> StandardOutput<sessions::Model>
    where
        C: ConnectionTrait,
    {
        let inserted = session
            .clone()
            .insert(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::created(inserted))
    }

    async fn find_by_id_and_device<C>(
        &self,
        session_id: &Uuid,
        device_fingerprint: &str,
        db: &C,
    ) -> StandardOutput<Option<sessions::Model>>
    where
        C: ConnectionTrait,
    {
        let session = Entity::find()
            .filter(Column::Id.eq(*session_id))
            .filter(Column::DeviceFingerprint.eq(device_fingerprint))
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(session, None))
    }

    async fn soft_delete_by_user_and_device<C>(
        &self,
        user_id: &Uuid,
        device_fingerprint: &str,
        db: &C,
    ) -> StandardOutput<()>
    where
        C: ConnectionTrait,
    {
        let sessions_update = sessions::ActiveModel {
            deleted_at: Set(Some(Utc::now().into())),
            ..Default::default()
        };

        let _ = Entity::update_many()
            .filter(Column::IdUser.eq(*user_id))
            .filter(Column::DeviceFingerprint.eq(device_fingerprint))
            .filter(Column::DeletedAt.is_null())
            .set(sessions_update)
            .exec(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple((), None))
    }

    async fn find_by_id<C>(
        &self,
        session_id: &Uuid,
        db: &C,
    ) -> StandardOutput<Option<sessions::Model>>
    where
        C: ConnectionTrait,
    {
        let session = Entity::find_by_id(*session_id)
            .filter(Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(session, None))
    }
}
