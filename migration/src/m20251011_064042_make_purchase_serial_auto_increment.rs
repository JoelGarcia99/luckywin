use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the existing serial column
        manager
            .alter_table(
                Table::alter()
                    .table(Purchases::Table)
                    .drop_column(Purchases::Serial)
                    .to_owned(),
            )
            .await?;

        // Add it back as bigserial (auto-increment)
        manager
            .alter_table(
                Table::alter()
                    .table(Purchases::Table)
                    .add_column(big_integer(Purchases::Serial).auto_increment().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the auto-increment serial column
        manager
            .alter_table(
                Table::alter()
                    .table(Purchases::Table)
                    .drop_column(Purchases::Serial)
                    .to_owned(),
            )
            .await?;

        // Add it back as regular bigint
        manager
            .alter_table(
                Table::alter()
                    .table(Purchases::Table)
                    .add_column(big_integer(Purchases::Serial).not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Purchases {
    Table,
    Serial,
}
