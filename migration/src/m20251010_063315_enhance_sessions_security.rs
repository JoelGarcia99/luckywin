use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Rename refresh_token to refresh_token_hash
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .rename_column(Sessions::RefreshToken, Sessions::RefreshTokenHash)
                    .to_owned(),
            )
            .await?;

        // Add device_fingerprint column
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .add_column(ColumnDef::new(Sessions::DeviceFingerprint).text().null())
                    .to_owned(),
            )
            .await?;

        // Add ip column (TEXT type)
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .add_column(ColumnDef::new(Sessions::Ip).text().null())
                    .to_owned(),
            )
            .await?;

        // Add user_agent column
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .add_column(ColumnDef::new(Sessions::UserAgent).text().null())
                    .to_owned(),
            )
            .await?;

        // Add last_used_at column
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .add_column(
                        ColumnDef::new(Sessions::LastUsedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add is_active column with default TRUE
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .add_column(
                        ColumnDef::new(Sessions::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        // Add unique constraint on refresh_token_hash
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_sessions_refresh_token_hash_unique")
                    .table(Sessions::Table)
                    .col(Sessions::RefreshTokenHash)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Remove old non-unique index on refresh_token
        manager
            .drop_index(
                Index::drop()
                    .name("idx_sessions_refresh_token")
                    .table(Sessions::Table)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop unique index
        manager
            .drop_index(
                Index::drop()
                    .name("idx_sessions_refresh_token_hash_unique")
                    .table(Sessions::Table)
                    .to_owned(),
            )
            .await?;

        // Recreate old index
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_sessions_refresh_token")
                    .table(Sessions::Table)
                    .col(Sessions::RefreshTokenHash)
                    .to_owned(),
            )
            .await?;

        // Drop added columns
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .drop_column(Sessions::IsActive)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .drop_column(Sessions::LastUsedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .drop_column(Sessions::UserAgent)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .drop_column(Sessions::Ip)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .drop_column(Sessions::DeviceFingerprint)
                    .to_owned(),
            )
            .await?;

        // Rename back to refresh_token
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .rename_column(Sessions::RefreshTokenHash, Sessions::RefreshToken)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Sessions {
    #[iden = "sessions"]
    Table,
    #[iden = "refresh_token"]
    RefreshToken,
    #[iden = "refresh_token_hash"]
    RefreshTokenHash,
    #[iden = "device_fingerprint"]
    DeviceFingerprint,
    #[iden = "ip"]
    Ip,
    #[iden = "user_agent"]
    UserAgent,
    #[iden = "last_used_at"]
    LastUsedAt,
    #[iden = "is_active"]
    IsActive,
}
