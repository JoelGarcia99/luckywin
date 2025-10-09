use sea_orm::{
    ConnectionTrait, Database, DatabaseTransaction, DbBackend, Statement, TransactionTrait, Value,
};
use std::env;
use uuid::Uuid;

/// Upserts a game record by title (idempotent - won't create duplicates on multiple runs)
async fn upsert_game(
    db: &DatabaseTransaction,
    title: &str,
    banner_url: Option<&str>,
    ticket_price: i32,
    finished_at: &str,
) -> Result<Uuid, sea_orm::DbErr> {
    // Try to fetch existing game by title
    if let Some(row) = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT id FROM games WHERE lower(title) = lower($1) AND deleted_at IS NULL",
            vec![Value::from(title)],
        ))
        .await?
    {
        let id: Uuid = row.try_get("", "id")?;
        return Ok(id);
    }

    // Create new game if it doesn't exist
    let new_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    db.execute(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        INSERT INTO games (id, title, banner_url, ticket_price, finished_at, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5::timestamptz, $6, $7)
        "#,
        vec![
            Value::from(new_id),
            Value::from(title),
            Value::from(banner_url.map(|s| s.to_string())),
            Value::from(ticket_price),
            Value::from(finished_at.to_string()),
            Value::from(now),
            Value::from(now),
        ],
    ))
    .await?;

    Ok(new_id)
}

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/luckywin".to_string());

    let db = Database::connect(&db_url).await?;
    let txn = db.begin().await?;

    // ── Seed Games ───────────────────────────────────────────────────────────
    println!("🎮 Seeding games...");

    let game1_id = upsert_game(
        &txn,
        "Lotería Navideña 2025",
        Some("https://example.com/banners/christmas-lottery-2025.png"),
        100, // ticket price in cents (or your currency's smallest unit)
        "2025-12-25 23:59:59+00",
    )
    .await?;
    println!("  ✅ Game created/found: {}", game1_id);

    let game2_id = upsert_game(
        &txn,
        "Super Sorteo de Año Nuevo",
        Some("https://example.com/banners/new-year-lottery-2025.png"),
        150,
        "2026-01-01 23:59:59+00",
    )
    .await?;
    println!("  ✅ Game created/found: {}", game2_id);

    let game3_id = upsert_game(
        &txn,
        "Lotería del Día de las Madres",
        Some("https://example.com/banners/mothers-day-lottery-2025.png"),
        75,
        "2025-05-11 23:59:59+00",
    )
    .await?;
    println!("  ✅ Game created/found: {}", game3_id);

    let game4_id = upsert_game(
        &txn,
        "Gran Sorteo de Verano 2025",
        None, // No banner URL
        200,
        "2025-08-15 23:59:59+00",
    )
    .await?;
    println!("  ✅ Game created/found: {}", game4_id);

    txn.commit().await?;

    println!("\n✅ Seed completed successfully!");
    println!("   - Games seeded: 4");
    println!("   - Running this seeder multiple times will NOT create duplicates");

    Ok(())
}
