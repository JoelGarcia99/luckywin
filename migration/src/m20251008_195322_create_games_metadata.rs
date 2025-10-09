use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(GamesMetadata::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(GamesMetadata::Id)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(GamesMetadata::IdGame).uuid().not_null())
            .col(
                ColumnDef::new(GamesMetadata::NumberLotteries)
                    .integer()
                    .not_null(),
            )
            .col(ColumnDef::new(GamesMetadata::TotalSold).integer().not_null())
            .col(
                ColumnDef::new(GamesMetadata::NumberWinners)
                    .integer()
                    .not_null(),
            )
            .col(ColumnDef::new(GamesMetadata::TotalPrizes).integer().not_null())
            .col(
                ColumnDef::new(GamesMetadata::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(GamesMetadata::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(GamesMetadata::DeletedAt).timestamp_with_time_zone())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_games_metadata_game_id")
                    .from(GamesMetadata::Table, GamesMetadata::IdGame)
                    .to(Games::Table, Games::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_games_metadata_id_game")
                    .table(GamesMetadata::Table)
                    .col(GamesMetadata::IdGame)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_games_metadata_deleted_at")
                    .table(GamesMetadata::Table)
                    .col(GamesMetadata::DeletedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GamesMetadata::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum GamesMetadata {
    #[iden = "games_metadata"]
    Table,
    Id,
    #[iden = "id_game"]
    IdGame,
    #[iden = "number_lotteries"]
    NumberLotteries,
    #[iden = "total_sold"]
    TotalSold,
    #[iden = "number_winners"]
    NumberWinners,
    #[iden = "total_prizes"]
    TotalPrizes,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "deleted_at"]
    DeletedAt,
}

#[derive(Iden)]
enum Games {
    #[iden = "games"]
    Table,
    Id,
}