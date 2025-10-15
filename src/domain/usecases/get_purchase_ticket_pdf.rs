use std::sync::Arc;

use chrono::{DateTime, Utc};
use chrono_tz::America::Bogota;
use reqwest::Client;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::outbound::{
        db::{games::GamesRepoImpl, purchases::PurchasesRepoImpl},
        pdf::gotenberg::send_html_to_gotenberg,
        rest::dtos::purchases::TicketOutputDto,
    },
    domain::repositories::{games::GamesRepo, purchases::PurchasesRepo},
    shared::{
        errors::Rfc7807Problem,
        templates::{LuckyNumberSet, LuckywinTicketContext, render_luckywin_ticket},
    },
};

pub struct GetPurchaseTicketPdfUseCase {
    purchases_repo: PurchasesRepoImpl,
    games_repo: GamesRepoImpl,
    db: Arc<DatabaseConnection>,
}

impl GetPurchaseTicketPdfUseCase {
    pub fn new(
        purchases_repo: PurchasesRepoImpl,
        games_repo: GamesRepoImpl,
        db: Arc<DatabaseConnection>,
    ) -> Self {
        Self {
            purchases_repo,
            games_repo,
            db,
        }
    }

    pub async fn execute(&self, purchase_id: &Uuid) -> Result<Vec<u8>, Box<Rfc7807Problem>> {
        let purchase = self
            .purchases_repo
            .find_by_id(purchase_id, &*self.db)
            .await?
            .data
            .ok_or_else(|| {
                Box::new(Rfc7807Problem::not_found(
                    Some("Compra no encontrada"),
                    Some("No existe una compra con el identificador proporcionado"),
                ))
            })?;

        let game = self
            .games_repo
            .get_by_id(&purchase.id_game, &*self.db)
            .await?
            .data;

        if purchase.tickets.is_empty() {
            return Err(Box::new(Rfc7807Problem::simple_bad_request(
                Some("Compra sin boletos"),
                Some("La compra seleccionada no tiene boletos asociados"),
                Some("PURCHASE_WITHOUT_TICKETS"),
            )));
        }

        let mut tickets = purchase.tickets.clone();
        tickets.sort_by_key(|t| t.serial);

        let context = LuckywinTicketContext {
            client_name: purchase.client_name.clone(),
            client_cellphone: purchase
                .client_cellphone
                .clone()
                .unwrap_or_else(|| "No registrado".into()),
            game_name: game.title.clone(),
            game_serial: game.id.to_string(),
            purchase_serial: format_serial(purchase.serial),
            purchase_date: format_datetime(purchase.created_at),
            game_closes_at: format_game_close(game.closes_at.map(|dt| dt.to_utc())),
            total_paid: format!("{:.2}", purchase.total_paid as f64 / 100.0),
            number_sets: build_number_sets(&tickets),
        };

        let html = render_luckywin_ticket(&context);
        let client = Client::new();

        send_html_to_gotenberg(&html, &client).await.map_err(|_| {
            Box::new(Rfc7807Problem::server_error(
                Some("No se pudo generar el PDF"),
                Some("Tuvimos un inconveniente al convertir el ticket"),
                Some("PDF_GENERATION_ERROR"),
            ))
        })
    }
}

fn format_datetime(datetime: DateTime<Utc>) -> String {
    let bogota_time = datetime.with_timezone(&Bogota);
    bogota_time.format("%Y-%m-%d %H:%M").to_string()
}

fn format_game_close(datetime: Option<DateTime<Utc>>) -> String {
    datetime
        .map(|dt| {
            let bogota_time = dt.with_timezone(&Bogota);
            bogota_time.format("%Y-%m-%d %H:%M").to_string()
        })
        .unwrap_or_else(|| "Pendiente por definir".into())
}

fn format_serial(serial: i64) -> String {
    format!("#{:05}", serial)
}

fn build_number_sets(tickets: &[TicketOutputDto]) -> Vec<LuckyNumberSet> {
    tickets
        .iter()
        .map(|ticket| LuckyNumberSet {
            title: format!("Serial {}", format_serial(ticket.serial)),
            digits: ticket
                .numbers
                .iter()
                .map(|number| format!("{:02}", number))
                .collect(),
        })
        .collect()
}
