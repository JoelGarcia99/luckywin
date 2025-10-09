use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Winners::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Winners::Id)
                    .uuid()
                    .not_null()
                    .primary_key(),
            )
            .col(ColumnDef::new(Winners::IdGame).uuid().not_null())
            .col(ColumnDef::new(Winners::IdClient).uuid().not_null())
            .col(ColumnDef::new(Winners::Matches).tiny_integer().not_null())
            .col(ColumnDef::new(Winners::TotalEarned).integer().not_null())
            .col(
                ColumnDef::new(Winners::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Winners::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null(),
            )
            .col(ColumnDef::new(Winners::DeletedAt).timestamp_with_time_zone())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_winners_game_id")
                    .from(Winners::Table, Winners::IdGame)
                    .to(Games::Table, Games::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .name("fk_winners_client_id")
                    .from(Winners::Table, Winners::IdClient)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_winners_id_game")
                    .table(Winners::Table)
                    .col(Winners::IdGame)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_winners_id_client")
                    .table(Winners::Table)
                    .col(Winners::IdClient)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_winners_deleted_at")
                    .table(Winners::Table)
                    .col(Winners::DeletedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Winners::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Winners {
    #[iden = "winners"]
    Table,
    Id,
    #[iden = "id_game"]
    IdGame,
    #[iden = "id_client"]
    IdClient,
    Matches,
    #[iden = "total_earned"]
    TotalEarned,
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

#[derive(Iden)]
enum Users {
    #[iden = "users"]
    Table,
    Id,
}