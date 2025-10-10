use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
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

    async fn update_status<C>(
        &self,
        lottery_id: Uuid,
        status: String,
        db: &C,
    ) -> StandardOutput<LotteryOutputDto>
    where
        C: ConnectionTrait,
    {
        // Find the lottery by ID, excluding soft-deleted records
        let lottery = lotteries::Entity::find_by_id(lottery_id)
            .filter(lotteries::Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?
            .ok_or_else(|| {
                Box::new(crate::shared::errors::Rfc7807Problem::not_found(
                    Some("Lotería no encontrada"),
                    Some("La lotería especificada no existe o ha sido eliminada"),
                ))
            })?;

        // Update the status
        let mut active_model: lotteries::ActiveModel = lottery.into();
        active_model.status = Set(status);
        active_model.updated_at = Set(chrono::Utc::now().into());

        let updated_lottery = active_model.update(db).await.map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(updated_lottery.into(), None))
    }
}
