use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create purchases table
        manager
            .create_table(
                Table::create()
                    .table(Purchases::Table)
                    .if_not_exists()
                    .col(uuid(Purchases::Id).primary_key())
                    .col(big_integer(Purchases::Serial).not_null())
                    .col(uuid(Purchases::IdGame).not_null())
                    .col(uuid(Purchases::IdClient).not_null())
                    .col(string(Purchases::ClientName).not_null())
                    .col(string_null(Purchases::ClientEmail))
                    .col(string_null(Purchases::ClientCellphone))
                    .col(string_null(Purchases::Location))
                    .col(integer(Purchases::TotalPaid).not_null())
                    .col(string(Purchases::Status).not_null())
                    .col(timestamp_with_time_zone(Purchases::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(Purchases::UpdatedAt).not_null())
                    .col(timestamp_with_time_zone_null(Purchases::DeletedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_purchases_game")
                            .from(Purchases::Table, Purchases::IdGame)
                            .to(Games::Table, Games::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_purchases_client")
                            .from(Purchases::Table, Purchases::IdClient)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Add index on game_id for faster queries
        manager
            .create_index(
                Index::create()
                    .name("idx_purchases_game_id")
                    .table(Purchases::Table)
                    .col(Purchases::IdGame)
                    .to_owned(),
            )
            .await?;

        // Add index on client_id
        manager
            .create_index(
                Index::create()
                    .name("idx_purchases_client_id")
                    .table(Purchases::Table)
                    .col(Purchases::IdClient)
                    .to_owned(),
            )
            .await?;

        // Add id_purchase column to lotteries table (nullable for now to allow data migration)
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_column(uuid_null(Lotteries::IdPurchase))
                    .to_owned(),
            )
            .await?;

        // Create foreign key for id_purchase
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_lotteries_purchase")
                            .from_tbl(Lotteries::Table)
                            .from_col(Lotteries::IdPurchase)
                            .to_tbl(Purchases::Table)
                            .to_col(Purchases::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        // Drop columns from lotteries table that are moving to purchases
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .drop_column(Lotteries::IdClient)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .drop_column(Lotteries::TotalPaid)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .drop_column(Lotteries::ClientName)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .drop_column(Lotteries::Location)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .drop_column(Lotteries::Cellphone)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .drop_column(Lotteries::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .drop_column(Lotteries::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Re-add columns to lotteries table
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_column(uuid(Lotteries::IdClient).not_null())
                    .add_column(integer(Lotteries::TotalPaid).not_null())
                    .add_column(string(Lotteries::ClientName).not_null())
                    .add_column(string_null(Lotteries::Location))
                    .add_column(string_null(Lotteries::Cellphone))
                    .add_column(string_null(Lotteries::Email))
                    .add_column(string(Lotteries::Status).not_null())
                    .to_owned(),
            )
            .await?;

        // Drop id_purchase column from lotteries
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .drop_column(Lotteries::IdPurchase)
                    .to_owned(),
            )
            .await?;

        // Drop purchases table
        manager
            .drop_table(Table::drop().table(Purchases::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Purchases {
    Table,
    Id,
    Serial,
    IdGame,
    IdClient,
    ClientName,
    ClientEmail,
    ClientCellphone,
    Location,
    TotalPaid,
    Status,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Lotteries {
    Table,
    IdPurchase,
    IdClient,
    TotalPaid,
    ClientName,
    Location,
    Cellphone,
    Email,
    Status,
}

#[derive(DeriveIden)]
enum Games {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
