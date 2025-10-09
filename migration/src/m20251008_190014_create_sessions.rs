use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Sessions::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Sessions::Id)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Sessions::IdUser).uuid().not_null())
            .col(ColumnDef::new(Sessions::RefreshToken).text().not_null())
            .col(
                ColumnDef::new(Sessions::ExpiresAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Sessions::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Sessions::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(Sessions::DeletedAt).timestamp_with_time_zone())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_sessions_user_id")
                    .from(Sessions::Table, Sessions::IdUser)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_sessions_refresh_token")
                    .table(Sessions::Table)
                    .col(Sessions::RefreshToken)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_sessions_deleted_at")
                    .table(Sessions::Table)
                    .col(Sessions::DeletedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Sessions {
    #[iden = "sessions"]
    Table,
    Id,
    #[iden = "id_user"]
    IdUser,
    #[iden = "refresh_token"]
    RefreshToken,
    #[iden = "expires_at"]
    ExpiresAt,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "deleted_at"]
    DeletedAt,
}

#[derive(Iden)]
enum Users {
    #[iden = "users"]
    Table,
    Id,
}