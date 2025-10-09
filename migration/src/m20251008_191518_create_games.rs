use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Games::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Games::Id)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Games::Title).string().not_null())
            .col(ColumnDef::new(Games::BannerUrl).string())
            .col(
                ColumnDef::new(Games::FinishedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Games::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Games::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(Games::DeletedAt).timestamp_with_time_zone())
            .to_owned();

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_games_finished_at")
                    .table(Games::Table)
                    .col(Games::FinishedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_games_deleted_at")
                    .table(Games::Table)
                    .col(Games::DeletedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Games::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Games {
    #[iden = "games"]
    Table,
    Id,
    Title,
    #[iden = "banner_url"]
    BannerUrl,
    #[iden = "finished_at"]
    FinishedAt,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "deleted_at"]
    DeletedAt,
}