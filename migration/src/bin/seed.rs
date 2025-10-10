use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
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

/// Hashes a password using Argon2 (default configuration)
fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Failed to hash password: {}", e))
}

/// Upserts an admin user by email (idempotent - won't create duplicates on multiple runs)
async fn upsert_admin_user(
    db: &DatabaseTransaction,
    first_name: &str,
    last_name: &str,
    location: &str,
    cellphone: &str,
    email: &str,
    password: &str,
    role: &str,
) -> Result<Uuid, sea_orm::DbErr> {
    // Try to fetch existing user by email
    if let Some(row) = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT id FROM users WHERE lower(email) = lower($1) AND deleted_at IS NULL",
            vec![Value::from(email)],
        ))
        .await?
    {
        let id: Uuid = row.try_get("", "id")?;
        println!("  ℹ️  Admin user already exists with email: {}", email);
        return Ok(id);
    }

    // Hash the password using Argon2i
    let password_hash = hash_password(password)
        .map_err(|e| sea_orm::DbErr::Custom(e))?;

    // Create new user if it doesn't exist
    let new_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    db.execute(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        INSERT INTO users (id, first_name, last_name, location, cellphone, email, password, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
        vec![
            Value::from(new_id),
            Value::from(first_name),
            Value::from(last_name),
            Value::from(location),
            Value::from(cellphone),
            Value::from(email),
            Value::from(password_hash),
            Value::from(role),
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

    // ── Seed Admin User ──────────────────────────────────────────────────────
    println!("👤 Seeding admin user...");

    let admin_id = upsert_admin_user(
        &txn,
        "Luckywin",
        "Administrador",
        "portoviejo",
        "xxxxxxxxxx",
        "dev@devallsas.com",
        "123Root.",
        "admin",
    )
    .await?;
    println!("  ✅ Admin user created/found: {}", admin_id);

    // ── Seed Games ───────────────────────────────────────────────────────────
    println!("\n🎮 Seeding games...");

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
    println!("   - Admin users seeded: 1");
    println!("   - Games seeded: 4");
    println!("   - Running this seeder multiple times will NOT create duplicates");
    println!("\n📧 Admin credentials:");
    println!("   Email: dev@devallsas.com");
    println!("   Cellphone: xxxxxxxxxx");
    println!("   Password: 123Root.");

    Ok(())
}
