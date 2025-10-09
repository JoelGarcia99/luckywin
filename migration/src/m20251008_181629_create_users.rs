use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Users::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Users::Id)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Users::FirstName).string().not_null())
            .col(ColumnDef::new(Users::LastName).string())
            .col(ColumnDef::new(Users::Location).string())
            .col(ColumnDef::new(Users::Cellphone).string().not_null())
            .col(ColumnDef::new(Users::Email).string().not_null())
            .col(ColumnDef::new(Users::Password).text())
            .col(ColumnDef::new(Users::Role).string().not_null())
            .col(
                ColumnDef::new(Users::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Users::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(Users::DeletedAt).timestamp_with_time_zone())
            .to_owned();

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_cellphone_deleted_at")
                    .table(Users::Table)
                    .col(Users::Cellphone)
                    .unique()
                    .and_where(Expr::col(Users::DeletedAt).is_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_email_deleted_at")
                    .table(Users::Table)
                    .col(Users::Email)
                    .unique()
                    .and_where(Expr::col(Users::DeletedAt).is_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_deleted_at")
                    .table(Users::Table)
                    .col(Users::DeletedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(Index::drop().name("idx_users_cellphone_deleted_at").to_owned()).await?;
        manager.drop_index(Index::drop().name("idx_users_email_deleted_at").to_owned()).await?;
        manager.drop_index(Index::drop().name("idx_users_deleted_at").to_owned()).await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    #[iden = "users"]
    Table,
    Id,
    #[iden = "first_name"]
    FirstName,
    #[iden = "last_name"]
    LastName,
    Location,
    Cellphone,
    Email,
    Password,
    Role,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "deleted_at"]
    DeletedAt,
}
