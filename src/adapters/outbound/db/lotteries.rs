use async_trait::async_trait;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    adapters::outbound::{
        db::error_parsing::process_db_err, rest::dtos::lottery::LotteryOutputDto,
    },
    domain::{entities::lotteries, repositories::lotteries::LotteriesRepo},
    shared::errors::{StandardOutput, StandardResponse},
};

#[derive(Clone, Copy)]
pub struct LotteriesRepoImpl;

#[async_trait]
impl LotteriesRepo for LotteriesRepoImpl {
    async fn batch_create<C>(
        &self,
        lotteries_models: Vec<lotteries::ActiveModel>,
        db: &C,
    ) -> StandardOutput<Vec<LotteryOutputDto>>
    where
        C: ConnectionTrait,
    {
        lotteries::Entity::insert_many(lotteries_models.clone())
            .exec(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        // getting batch by IDs
        self.list_batch_by_ids(
            lotteries_models
                .clone()
                .into_iter()
                // it's secure to unwrap here since it's controlled
                .map(|e| *e.id.into_value().unwrap().as_ref_uuid().unwrap())
                .collect::<Vec<Uuid>>(),
            db,
        )
        .await
    }

    async fn list_batch_by_ids<C>(
        &self,
        ids: Vec<Uuid>,
        db: &C,
    ) -> StandardOutput<Vec<LotteryOutputDto>>
    where
        C: ConnectionTrait,
    {
        let lotteries = lotteries::Entity::find()
            .filter(lotteries::Column::Id.is_in(ids))
            .all(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(
            lotteries
                .into_iter()
                .map(|lottery| lottery.into())
                .collect::<Vec<LotteryOutputDto>>(),
            None,
        ))
    }
}
