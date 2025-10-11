use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add index on purchases.status for filtering paid/unpaid purchases
        manager
            .create_index(
                Index::create()
                    .name("idx_purchases_status")
                    .table(Purchases::Table)
                    .col(Purchases::Status)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Add index on purchases.deleted_at for soft-delete filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_purchases_deleted_at")
                    .table(Purchases::Table)
                    .col(Purchases::DeletedAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Add index on lotteries.id_purchase for joining lotteries with purchases
        manager
            .create_index(
                Index::create()
                    .name("idx_lotteries_id_purchase")
                    .table(Lotteries::Table)
                    .col(Lotteries::IdPurchase)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Add index on lotteries.deleted_at for soft-delete filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_lotteries_deleted_at")
                    .table(Lotteries::Table)
                    .col(Lotteries::DeletedAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Add index on games.deleted_at for soft-delete filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_games_deleted_at")
                    .table(Games::Table)
                    .col(Games::DeletedAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Add index on games.finished_at for active game queries
        manager
            .create_index(
                Index::create()
                    .name("idx_games_finished_at")
                    .table(Games::Table)
                    .col(Games::FinishedAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Add composite index on purchases for common query pattern (game_id, status, deleted_at)
        manager
            .create_index(
                Index::create()
                    .name("idx_purchases_game_status_deleted")
                    .table(Purchases::Table)
                    .col(Purchases::IdGame)
                    .col(Purchases::Status)
                    .col(Purchases::DeletedAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop all created indexes
        manager
            .drop_index(Index::drop().name("idx_purchases_status").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_purchases_deleted_at").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_lotteries_id_purchase").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_lotteries_deleted_at").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_games_deleted_at").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_games_finished_at").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_purchases_game_status_deleted").to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Purchases {
    Table,
    IdGame,
    Status,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Lotteries {
    Table,
    IdPurchase,
    DeletedAt,
}

#[derive(DeriveIden)]
enum Games {
    Table,
    DeletedAt,
    FinishedAt,
}
