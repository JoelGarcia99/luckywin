use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Change numbers column from integer (int4) to NUMERIC to support 128-bit integers
        // NUMERIC(39,0) can hold values up to 2^128 (340282366920938463463374607431768211455)
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .modify_column(
                        ColumnDef::new(Lotteries::Numbers)
                            .decimal_len(39, 0)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Revert numbers column from NUMERIC back to integer (int4)
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .modify_column(ColumnDef::new(Lotteries::Numbers).integer().not_null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Lotteries {
    #[iden = "lotteries"]
    Table,
    Numbers,
}
