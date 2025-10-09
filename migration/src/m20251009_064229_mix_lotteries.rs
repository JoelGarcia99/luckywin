use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop client_lotteries table (this will also drop all indexes and foreign keys)
        manager
            .drop_table(Table::drop().table(ClientLotteries::Table).to_owned())
            .await?;

        // Add new columns to lotteries table from client_lotteries
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_column(ColumnDef::new(Lotteries::IdClient).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_column(ColumnDef::new(Lotteries::TotalPaid).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_column(
                        ColumnDef::new(Lotteries::ClientName)
                            .string_len(255)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_column(ColumnDef::new(Lotteries::Location).string_len(255))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_column(ColumnDef::new(Lotteries::Cellphone).string_len(255))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_column(ColumnDef::new(Lotteries::Email).string_len(255))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_column(ColumnDef::new(Lotteries::Status).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Create indexes for email and cellphone
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_lotteries_email")
                    .table(Lotteries::Table)
                    .col(Lotteries::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_lotteries_cellphone")
                    .table(Lotteries::Table)
                    .col(Lotteries::Cellphone)
                    .to_owned(),
            )
            .await?;

        // Add foreign key to users table
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("fk_lotteries_client_id")
                            .from_tbl(Lotteries::Table)
                            .from_col(Lotteries::IdClient)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop foreign key
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .drop_foreign_key(Alias::new("fk_lotteries_client_id"))
                    .to_owned(),
            )
            .await?;

        // Drop indexes
        manager
            .drop_index(
                Index::drop()
                    .name("idx_lotteries_cellphone")
                    .table(Lotteries::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_lotteries_email")
                    .table(Lotteries::Table)
                    .to_owned(),
            )
            .await?;

        // Drop columns from lotteries table
        manager
            .alter_table(
                Table::alter()
                    .table(Lotteries::Table)
                    .drop_column(Lotteries::Status)
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
                    .drop_column(Lotteries::Cellphone)
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
                    .drop_column(Lotteries::ClientName)
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
                    .drop_column(Lotteries::IdClient)
                    .to_owned(),
            )
            .await?;

        // Recreate client_lotteries table
        let table = Table::create()
            .table(ClientLotteries::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ClientLotteries::Id)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(ClientLotteries::IdClient).uuid().not_null())
            .col(ColumnDef::new(ClientLotteries::IdLottery).uuid().not_null())
            .col(
                ColumnDef::new(ClientLotteries::TotalPaid)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ClientLotteries::ClientName)
                    .string()
                    .not_null(),
            )
            .col(ColumnDef::new(ClientLotteries::Location).string())
            .col(ColumnDef::new(ClientLotteries::Cellphone).string())
            .col(ColumnDef::new(ClientLotteries::Email).string())
            .col(
                ColumnDef::new(ClientLotteries::Status)
                    .string_len(100)
                    .not_null(),
            )
            .col(
                ColumnDef::new(ClientLotteries::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(ClientLotteries::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(ClientLotteries::DeletedAt).timestamp_with_time_zone())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_client_lotteries_client_id")
                    .from(ClientLotteries::Table, ClientLotteries::IdClient)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_client_lotteries_lottery_id")
                    .from(ClientLotteries::Table, ClientLotteries::IdLottery)
                    .to(Lotteries::Table, Lotteries::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_client_lotteries_email")
                    .table(ClientLotteries::Table)
                    .col(ClientLotteries::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_client_lotteries_cellphone")
                    .table(ClientLotteries::Table)
                    .col(ClientLotteries::Cellphone)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_client_lotteries_deleted_at")
                    .table(ClientLotteries::Table)
                    .col(ClientLotteries::DeletedAt)
                    .to_owned(),
            )
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
    #[iden = "id_client"]
    IdClient,
    Serial,
    Numbers,
    #[iden = "total_paid"]
    TotalPaid,
    #[iden = "client_name"]
    ClientName,
    Location,
    Cellphone,
    Email,
    Status,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "deleted_at"]
    DeletedAt,
}

#[derive(Iden)]
enum ClientLotteries {
    #[iden = "client_lotteries"]
    Table,
    Id,
    #[iden = "id_client"]
    IdClient,
    #[iden = "id_lottery"]
    IdLottery,
    #[iden = "total_paid"]
    TotalPaid,
    #[iden = "client_name"]
    ClientName,
    Location,
    Cellphone,
    Email,
    Status,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "deleted_at"]
    DeletedAt,
}

#[derive(Iden)]
enum Users {
    #[iden = "users"]
    Table,
    Id,
}
