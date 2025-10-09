use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Lotteries::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Lotteries::Id)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Lotteries::IdGame).uuid().not_null())
            .col(
                ColumnDef::new(Lotteries::Serial)
                    .big_integer()
                    .auto_increment()
                    .not_null(),
            )
            .col(ColumnDef::new(Lotteries::Numbers).integer().not_null())
            .col(
                ColumnDef::new(Lotteries::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Lotteries::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(Lotteries::DeletedAt).timestamp_with_time_zone())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_lotteries_game_id")
                    .from(Lotteries::Table, Lotteries::IdGame)
                    .to(Games::Table, Games::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_lotteries_deleted_at")
                    .table(Lotteries::Table)
                    .col(Lotteries::DeletedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Lotteries::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Lotteries {
    #[iden = "lotteries"]
    Table,
    Id,
    #[iden = "id_game"]
    IdGame,
    Serial,
    Numbers,
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