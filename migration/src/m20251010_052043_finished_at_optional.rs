use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Make finished_at column nullable with NULL as default value
        manager
            .alter_table(
                Table::alter()
                    .table(Games::Table)
                    .modify_column(
                        ColumnDef::new(Games::FinishedAt)
                            .timestamp_with_time_zone()
                            .null()
                            .default(Option::<String>::None),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Revert finished_at column back to NOT NULL
        manager
            .alter_table(
                Table::alter()
                    .table(Games::Table)
                    .modify_column(
                        ColumnDef::new(Games::FinishedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Games {
    #[iden = "games"]
    Table,
    #[iden = "finished_at"]
    FinishedAt,
}
