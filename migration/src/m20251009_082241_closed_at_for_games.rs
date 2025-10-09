use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add closed_at field to games table (nullable timestamp)
        manager
            .alter_table(
                Table::alter()
                    .table(Games::Table)
                    .add_column(ColumnDef::new(Games::ClosedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop closed_at field from games table
        manager
            .alter_table(
                Table::alter()
                    .table(Games::Table)
                    .drop_column(Games::ClosedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Games {
    #[iden = "games"]
    Table,
    #[iden = "closed_at"]
    ClosedAt,
}
