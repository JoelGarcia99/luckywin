use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
    adapters::outbound::{
        db::error_parsing::process_db_err,
        rest::dtos::purchases::{PurchaseOutputDto, TicketOutputDto},
    },
    domain::{
        entities::{lotteries, purchases},
        enums::lottery_status::LotteryStatus,
        repositories::purchases::PurchasesRepo,
    },
    shared::{
        errors::{PaginatorResponse, Rfc7807Problem, StandardOutput, StandardResponse},
        utils::bitboards,
    },
};

#[derive(Clone, Copy)]
pub struct PurchasesRepoImpl;

#[async_trait]
impl PurchasesRepo for PurchasesRepoImpl {
    async fn create<C>(
        &self,
        id_game: &Uuid,
        id_client: &Uuid,
        client_name: &str,
        client_email: Option<&str>,
        client_cellphone: Option<&str>,
        location: Option<&str>,
        total_paid: i32,
        db: &C,
    ) -> StandardOutput<PurchaseOutputDto>
    where
        C: ConnectionTrait,
    {
        let new_purchase = purchases::ActiveModel {
            id: Set(Uuid::new_v4()),
            id_game: Set(*id_game),
            id_client: Set(*id_client),
            client_name: Set(client_name.to_string()),
            client_email: Set(client_email.map(|s| s.to_string())),
            client_cellphone: Set(client_cellphone.map(|s| s.to_string())),
            location: Set(location.map(|s| s.to_string())),
            total_paid: Set(total_paid),
            status: Set(LotteryStatus::Unpaid.to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
            deleted_at: Set(None),
            ..Default::default() // Let serial auto-increment
        };

        let created_purchase = new_purchase
            .insert(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(
            PurchaseOutputDto {
                id: created_purchase.id,
                serial: created_purchase.serial,
                id_game: created_purchase.id_game,
                id_client: created_purchase.id_client,
                client_name: created_purchase.client_name,
                client_email: created_purchase.client_email,
                client_cellphone: created_purchase.client_cellphone,
                location: created_purchase.location,
                total_paid: created_purchase.total_paid,
                status: created_purchase.status,
                tickets: vec![], // Empty initially, will be populated after tickets are created
                created_at: created_purchase.created_at.into(),
                updated_at: created_purchase.updated_at.into(),
            },
            None,
        ))
    }

    async fn find_by_id<C>(&self, id: &Uuid, db: &C) -> StandardOutput<Option<PurchaseOutputDto>>
    where
        C: ConnectionTrait,
    {
        let purchase = purchases::Entity::find_by_id(*id)
            .filter(purchases::Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        match purchase {
            Some(p) => {
                // Get associated lottery tickets
                let tickets = lotteries::Entity::find()
                    .filter(lotteries::Column::IdPurchase.eq(p.id))
                    .filter(lotteries::Column::DeletedAt.is_null())
                    .order_by_asc(lotteries::Column::Serial)
                    .all(db)
                    .await
                    .map_err(|e| process_db_err(&e))?;

                Ok(StandardResponse::simple(
                    Some(PurchaseOutputDto {
                        id: p.id,
                        serial: p.serial,
                        id_game: p.id_game,
                        id_client: p.id_client,
                        client_name: p.client_name,
                        client_email: p.client_email,
                        client_cellphone: p.client_cellphone,
                        location: p.location,
                        total_paid: p.total_paid,
                        status: p.status,
                        tickets: tickets
                            .into_iter()
                            .map(|t| TicketOutputDto {
                                id: t.id,
                                serial: t.serial,
                                numbers: bitboards::parse_bitboard_into_vec(t.numbers)
                                    .unwrap_or_default(),
                                created_at: t.created_at.into(),
                                updated_at: t.updated_at.into(),
                            })
                            .collect(),
                        created_at: p.created_at.into(),
                        updated_at: p.updated_at.into(),
                    }),
                    None,
                ))
            }
            None => Ok(StandardResponse::simple(None, None)),
        }
    }

    async fn mark_as_paid<C>(&self, id: &Uuid, db: &C) -> StandardOutput<PurchaseOutputDto>
    where
        C: ConnectionTrait,
    {
        let purchase = purchases::Entity::find_by_id(*id)
            .filter(purchases::Column::DeletedAt.is_null())
            .one(db)
            .await
            .map_err(|e| process_db_err(&e))?
            .ok_or_else(|| {
                Box::new(Rfc7807Problem::not_found(
                    Some("Compra no encontrada"),
                    Some("La compra especificada no existe o ha sido eliminada"),
                ))
            })?;

        if purchase.status == LotteryStatus::Paid.to_string() {
            return Err(Box::new(Rfc7807Problem::simple_bad_request(
                Some("Compra ya pagada"),
                Some("Esta compra ya ha sido marcada como pagada"),
                Some("PURCHASE_ALREADY_PAID"),
            )));
        }

        let mut active_model: purchases::ActiveModel = purchase.into();
        active_model.status = Set(LotteryStatus::Paid.to_string());
        active_model.updated_at = Set(chrono::Utc::now().into());

        let updated_purchase = active_model
            .update(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        // Get associated lottery tickets
        let tickets = lotteries::Entity::find()
            .filter(lotteries::Column::IdPurchase.eq(updated_purchase.id))
            .filter(lotteries::Column::DeletedAt.is_null())
            .order_by_asc(lotteries::Column::Serial)
            .all(db)
            .await
            .map_err(|e| process_db_err(&e))?;

        Ok(StandardResponse::simple(
            PurchaseOutputDto {
                id: updated_purchase.id,
                serial: updated_purchase.serial,
                id_game: updated_purchase.id_game,
                id_client: updated_purchase.id_client,
                client_name: updated_purchase.client_name,
                client_email: updated_purchase.client_email,
                client_cellphone: updated_purchase.client_cellphone,
                location: updated_purchase.location,
                total_paid: updated_purchase.total_paid,
                status: updated_purchase.status,
                tickets: tickets
                    .into_iter()
                    .map(|t| TicketOutputDto {
                        id: t.id,
                        serial: t.serial,
                        numbers: bitboards::parse_bitboard_into_vec(t.numbers).unwrap_or_default(),
                        created_at: t.created_at.into(),
                        updated_at: t.updated_at.into(),
                    })
                    .collect(),
                created_at: updated_purchase.created_at.into(),
                updated_at: updated_purchase.updated_at.into(),
            },
            None,
        ))
    }

    async fn find_by_game_with_tickets<C>(
        &self,
        game_id: &Uuid,
        page: u64,
        per_page: u64,
        serial_filter: Option<i64>,
        db: &C,
    ) -> StandardOutput<Vec<PurchaseOutputDto>>
    where
        C: ConnectionTrait,
    {
        // Build the base query
        let mut query = purchases::Entity::find()
            .filter(purchases::Column::IdGame.eq(*game_id))
            .filter(purchases::Column::DeletedAt.is_null());

        // Apply serial filter if provided
        if let Some(serial) = serial_filter {
            query = query.filter(purchases::Column::Serial.eq(serial));
        }

        // Order by created_at descending
        let query = query.order_by_desc(purchases::Column::CreatedAt);

        // Create paginator
        let paginator = query.paginate(db, per_page);

        // Fetch total count and data in parallel
        let total_records_promise = paginator.num_items();
        let data_promise = paginator.fetch_page(page);

        let (total_records, purchases_data) =
            match tokio::join!(total_records_promise, data_promise) {
                (Ok(total), Ok(data)) => (total, data),
                (Err(e), _) => return Err(process_db_err(&e)),
                (_, Err(e)) => return Err(process_db_err(&e)),
            };

        // For each purchase, get its lottery tickets
        let mut results = Vec::new();
        for purchase in purchases_data {
            let tickets = lotteries::Entity::find()
                .filter(lotteries::Column::IdPurchase.eq(purchase.id))
                .filter(lotteries::Column::DeletedAt.is_null())
                .order_by_asc(lotteries::Column::Serial)
                .all(db)
                .await
                .map_err(|e| process_db_err(&e))?;

            results.push(PurchaseOutputDto {
                id: purchase.id,
                serial: purchase.serial,
                id_game: purchase.id_game,
                id_client: purchase.id_client,
                client_name: purchase.client_name,
                client_email: purchase.client_email,
                client_cellphone: purchase.client_cellphone,
                location: purchase.location,
                total_paid: purchase.total_paid,
                status: purchase.status,
                tickets: tickets
                    .into_iter()
                    .map(|t| TicketOutputDto {
                        id: t.id,
                        serial: t.serial,
                        numbers: bitboards::parse_bitboard_into_vec(t.numbers).unwrap_or_default(),
                        created_at: t.created_at.into(),
                        updated_at: t.updated_at.into(),
                    })
                    .collect(),
                created_at: purchase.created_at.into(),
                updated_at: purchase.updated_at.into(),
            });
        }

        let paginator = PaginatorResponse::new(total_records, page, per_page);

        Ok(StandardResponse::with_paginator(results, paginator, None))
    }
}
