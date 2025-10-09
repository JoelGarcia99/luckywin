use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add ticket_price field to games table
        manager
            .alter_table(
                Table::alter()
                    .table(Games::Table)
                    .add_column(ColumnDef::new(Games::TicketPrice).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop ticket_price field from games table
        manager
            .alter_table(
                Table::alter()
                    .table(Games::Table)
                    .drop_column(Games::TicketPrice)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Games {
    #[iden = "games"]
    Table,
    #[iden = "ticket_price"]
    TicketPrice,
}
