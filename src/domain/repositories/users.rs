use async_trait::async_trait;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

use crate::{domain::entities::users, shared::errors::StandardOutput};

#[async_trait]
pub trait UsersRepo: Send + Sync + Copy {
    /// Finds a user by email address
    /// Returns None if user is not found or has been soft-deleted
    async fn find_by_email<C>(&self, email: &str, db: &C) -> StandardOutput<Option<users::Model>>
    where
        C: ConnectionTrait;

    /// Finds a user by cellphone number
    /// Returns None if user is not found or has been soft-deleted
    async fn find_by_cellphone<C>(
        &self,
        cellphone: &str,
        db: &C,
    ) -> StandardOutput<Option<users::Model>>
    where
        C: ConnectionTrait;

    /// Finds a user by ID
    /// Returns None if user is not found or has been soft-deleted
    async fn find_by_id<C>(&self, id: &Uuid, db: &C) -> StandardOutput<Option<users::Model>>
    where
        C: ConnectionTrait;

    /// .
    async fn update_user<C>(
        &self,
        user: &users::ActiveModel,
        db: &C,
    ) -> StandardOutput<users::Model>
    where
        C: ConnectionTrait;
}
