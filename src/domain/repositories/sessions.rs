use async_trait::async_trait;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

use crate::{domain::entities::sessions, shared::errors::StandardOutput};

#[async_trait]
pub trait SessionsRepo: Send + Sync + Copy {
    /// Creates a new session in the database
    async fn create<C>(
        &self,
        session: &sessions::ActiveModel,
        db: &C,
    ) -> StandardOutput<sessions::Model>
    where
        C: ConnectionTrait;

    /// Finds an active session by ID and device fingerprint
    /// Returns None if session is not found or has been soft-deleted
    async fn find_by_id_and_device<C>(
        &self,
        session_id: &Uuid,
        device_fingerprint: &str,
        db: &C,
    ) -> StandardOutput<Option<sessions::Model>>
    where
        C: ConnectionTrait;

    /// Soft deletes all active sessions for a given user and device fingerprint
    /// Used for session rotation on refresh and logout
    async fn soft_delete_by_user_and_device<C>(
        &self,
        user_id: &Uuid,
        device_fingerprint: &str,
        db: &C,
    ) -> StandardOutput<()>
    where
        C: ConnectionTrait;

    /// Finds an active session by ID only (without device fingerprint check)
    /// Used for basic session validation
    async fn find_by_id<C>(
        &self,
        session_id: &Uuid,
        db: &C,
    ) -> StandardOutput<Option<sessions::Model>>
    where
        C: ConnectionTrait;
}
