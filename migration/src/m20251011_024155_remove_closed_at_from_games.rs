use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove closed_at column from games table
        manager
            .alter_table(
                Table::alter()
                    .table(Games::Table)
                    .drop_column(Games::ClosedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add back closed_at column if migration is rolled back
        manager
            .alter_table(
                Table::alter()
                    .table(Games::Table)
                    .add_column(timestamp_with_time_zone_null(Games::ClosedAt))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Games {
    Table,
    ClosedAt,
}
