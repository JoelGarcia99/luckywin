use std::sync::Arc;

use chrono::{TimeZone, Utc};
use sea_orm::{ActiveValue::Set, ConnectionTrait, TransactionTrait};
use uuid::Uuid;

use crate::{
    adapters::inbounds::rest::dtos::auth::LoginOutputDto,
    config::env::SecretsVariables,
    domain::{
        entities::{sessions, users},
        repositories::{sessions::SessionsRepo, users::UsersRepo},
    },
    shared::{
        errors::{
            CANNOT_DECODE_TOKEN, CANNOT_FIND_SESSION, Rfc7807Problem, StandardOutput,
            StandardResponse,
        },
        payloads::{AccessTokenPayload, RefreshTokenPayload},
        utils::{
            jwt::{decode_jwt, encode_jwt},
            security::{hash_password, verify_password},
        },
    },
};

const CORRUPTED_SESSION_MSG: &str = "La sesión es inválida o ha expirado";

pub struct RefreshTokenUseCase<
    UR: UsersRepo,
    SR: SessionsRepo,
    C: ConnectionTrait + TransactionTrait,
> {
    users_repo: UR,
    sessions_repo: SR,
    db: Arc<C>,
    secrets: SecretsVariables,
}

impl<UR: UsersRepo + Clone, SR: SessionsRepo + Clone, C: ConnectionTrait + TransactionTrait>
    RefreshTokenUseCase<UR, SR, C>
{
    pub fn new(users_repo: UR, sessions_repo: SR, db: Arc<C>, secrets: SecretsVariables) -> Self {
        Self {
            users_repo,
            sessions_repo,
            db,
            secrets,
        }
    }

    pub async fn execute(
        &self,
        access_token: &str,
        refresh_token: &str,
        device_fingerprint: &str,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> StandardOutput<LoginOutputDto> {
        // Decode access token (bypass expiration since it might be expired)
        let payload =
            decode_jwt::<AccessTokenPayload>(access_token, &self.secrets, true).map_err(|_| {
                Box::new(Rfc7807Problem::unauthorized(
                    Some("No se pudo refrescar su sesión"),
                    None,
                    Some(CANNOT_DECODE_TOKEN),
                ))
            })?;

        // Validate device fingerprint matches
        if payload.dfp != device_fingerprint {
            return Err(Box::new(Rfc7807Problem::unauthorized(
                Some("No se pudo refrescar su sesión"),
                Some(CORRUPTED_SESSION_MSG),
                Some(CANNOT_FIND_SESSION),
            )));
        }

        // Find session
        let session = self
            .sessions_repo
            .find_by_id_and_device(&payload.sid, device_fingerprint, &*self.db)
            .await?
            .data
            .ok_or_else(|| {
                Box::new(Rfc7807Problem::unauthorized(
                    Some("No se pudo refrescar su sesión"),
                    Some(CORRUPTED_SESSION_MSG),
                    Some(CANNOT_FIND_SESSION),
                ))
            })?;

        // Check if session is active
        if !session.is_active {
            return Err(Box::new(Rfc7807Problem::unauthorized(
                Some("Su sesión ha sido desactivada"),
                Some(CORRUPTED_SESSION_MSG),
                Some(CANNOT_FIND_SESSION),
            )));
        }

        // Check if session is expired
        let current_date: chrono::DateTime<chrono::FixedOffset> = Utc::now().into();
        if session.expires_at < current_date {
            return Err(Box::new(Rfc7807Problem::unauthorized(
                Some("Su sesión ha expirado"),
                Some(CORRUPTED_SESSION_MSG),
                Some(CANNOT_FIND_SESSION),
            )));
        }

        // Verify refresh token
        let passwords_match = verify_password(refresh_token, &session.refresh_token_hash)?;
        if !passwords_match {
            return Err(Box::new(Rfc7807Problem::unauthorized(
                Some("Token de refresco inválido"),
                Some(CORRUPTED_SESSION_MSG),
                Some(CANNOT_FIND_SESSION),
            )));
        }

        // Find user
        let user = self
            .users_repo
            .find_by_id(&session.id_user, &*self.db)
            .await?
            .data
            .ok_or_else(|| {
                Box::new(Rfc7807Problem::unauthorized(
                    Some("No se pudo refrescar su sesión"),
                    Some(CORRUPTED_SESSION_MSG),
                    Some(CANNOT_FIND_SESSION),
                ))
            })?;

        // Generate new tokens and rotate session
        self.save_session(&user, device_fingerprint, ip, user_agent)
            .await
    }

    async fn save_session(
        &self,
        user: &users::Model,
        device_fingerprint: &str,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> StandardOutput<LoginOutputDto> {
        let now = Utc::now().timestamp();

        let rt_exp_secs = self
            .secrets
            .jwt_rt_exp
            .parse::<i64>()
            .expect("Invalid JwtRtExp format");

        let rt_claim = RefreshTokenPayload {
            exp: now + rt_exp_secs,
        };

        // Start transaction
        let txn = self.db.as_ref().begin().await.map_err(|e| {
            tracing::error!("Failed to start transaction: {}", e);
            Box::new(Rfc7807Problem::server_error(
                Some("Error al refrescar sesión"),
                Some("No se pudo procesar la solicitud"),
                None,
            ))
        })?;

        // Soft delete old sessions for this user and device (token rotation)
        self.sessions_repo
            .soft_delete_by_user_and_device(&user.id, device_fingerprint, &txn)
            .await?;

        // Generate new refresh token
        let refresh_token = encode_jwt::<RefreshTokenPayload>(&rt_claim, &self.secrets)?;

        // Hash the refresh token
        let hashed_refresh_token = hash_password(&refresh_token)?;

        // Calculate expires_at
        let expires_at_utc = chrono::Utc
            .timestamp_opt(rt_claim.exp, 0)
            .single()
            .expect("Invalid timestamp");
        let expires_at = expires_at_utc.into();

        // Create new session
        let session_id = Uuid::new_v4();
        let session = sessions::ActiveModel {
            id: Set(session_id),
            id_user: Set(user.id),
            refresh_token_hash: Set(hashed_refresh_token),
            device_fingerprint: Set(Some(device_fingerprint.to_string())),
            ip: Set(ip.map(|s| s.to_string())),
            user_agent: Set(user_agent.map(|s| s.to_string())),
            expires_at: Set(expires_at),
            is_active: Set(true),
            last_used_at: Set(Some(Utc::now().into())),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        };

        self.sessions_repo.create(&session, &txn).await?;

        // Commit transaction
        txn.commit().await.map_err(|e| {
            tracing::error!("Failed to commit transaction: {}", e);
            Box::new(Rfc7807Problem::server_error(
                Some("Error al refrescar sesión"),
                Some("No se pudo guardar la sesión"),
                None,
            ))
        })?;

        // Generate new access token
        let at_claim = AccessTokenPayload {
            exp: now + self.secrets.jwt_at_exp.parse::<i64>().unwrap(),
            aud: self.secrets.jwt_aud.clone(),
            sub: user.id,
            iat: now,
            sid: session_id,
            dfp: device_fingerprint.to_string(),
            role: user.role.clone(),
        };

        let access_token = encode_jwt::<AccessTokenPayload>(&at_claim, &self.secrets)?;

        Ok(StandardResponse::simple(
            LoginOutputDto {
                access_token,
                refresh_token,
            },
            Some("Sesión refrescada correctamente"),
        ))
    }
}
