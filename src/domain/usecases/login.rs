use std::sync::Arc;

use chrono::{TimeZone, Utc};
use sea_orm::{ActiveValue::Set, ConnectionTrait, TransactionTrait};
use uuid::Uuid;

use crate::{
    adapters::inbounds::rest::dtos::auth::{LoginInputDto, LoginOutputDto},
    config::env::SecretsVariables,
    domain::{
        entities::{sessions, users},
        repositories::{sessions::SessionsRepo, users::UsersRepo},
    },
    shared::{
        errors::{INVALID_CREDENTIALS, Rfc7807Problem, StandardOutput, StandardResponse},
        payloads::{AccessTokenPayload, RefreshTokenPayload},
        utils::{
            jwt::encode_jwt,
            security::{hash_password, verify_password},
        },
    },
};

pub struct LoginUseCase<UR: UsersRepo, SR: SessionsRepo, C: ConnectionTrait + TransactionTrait> {
    users_repo: UR,
    sessions_repo: SR,
    db: Arc<C>,
    secrets: SecretsVariables,
}

impl<UR: UsersRepo + Clone, SR: SessionsRepo + Clone, C: ConnectionTrait + TransactionTrait>
    LoginUseCase<UR, SR, C>
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
        dto: &LoginInputDto,
        device_fingerprint: &str,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> StandardOutput<LoginOutputDto> {
        // Find user by email
        let user = self
            .users_repo
            .find_by_email(&dto.email, &*self.db)
            .await?
            .data
            .ok_or_else(|| {
                Box::new(Rfc7807Problem::simple_bad_request(
                    Some("Credenciales incorrectas"),
                    Some("Verifique su email y contraseña"),
                    Some(INVALID_CREDENTIALS),
                ))
            })?;

        // Check if user has a password set
        let password_hash = user.password.clone().ok_or_else(|| {
            Box::new(Rfc7807Problem::simple_bad_request(
                Some("Usuario sin contraseña"),
                Some("Este usuario no tiene una contraseña configurada"),
                Some(INVALID_CREDENTIALS),
            ))
        })?;

        // Verify password
        let passwords_match = verify_password(&dto.password, &password_hash)?;
        if !passwords_match {
            return Err(Box::new(Rfc7807Problem::simple_bad_request(
                Some("Credenciales incorrectas"),
                Some("Verifique su email y contraseña"),
                Some(INVALID_CREDENTIALS),
            )));
        }

        // Save session and generate tokens
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
                Some("Error al iniciar sesión"),
                Some("No se pudo procesar la solicitud"),
                None,
            ))
        })?;

        // Soft delete old sessions for this user and device
        self.sessions_repo
            .soft_delete_by_user_and_device(&user.id, device_fingerprint, &txn)
            .await?;

        // Generate refresh token
        let refresh_token = encode_jwt::<RefreshTokenPayload>(&rt_claim, &self.secrets)?;

        // Hash the refresh token
        let hashed_refresh_token = hash_password(&refresh_token)?;

        // Calculate expires_at
        let expires_at_utc = chrono::Utc
            .timestamp_opt(rt_claim.exp, 0)
            .single()
            .expect("Invalid timestamp");
        let expires_at = expires_at_utc.into();

        // Create session
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
                Some("Error al iniciar sesión"),
                Some("No se pudo guardar la sesión"),
                None,
            ))
        })?;

        // Generate access token
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
            Some("Sesión iniciada correctamente"),
        ))
    }
}
