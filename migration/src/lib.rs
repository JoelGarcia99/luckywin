pub use sea_orm_migration::prelude::*;

mod m20251008_181629_create_users;
mod m20251008_190014_create_sessions;
mod m20251008_191518_create_games;
mod m20251008_191905_create_lotteries;
mod m20251008_193237_create_client_lotteries;
mod m20251008_193817_create_winners;
mod m20251008_195322_create_games_metadata;
mod m20251009_064229_mix_lotteries;
mod m20251009_080515_ticket_price_to_game;
mod m20251009_082241_closed_at_for_games;
mod m20251009_175344_numbers_datatype_128_bits;
mod m20251010_052043_finished_at_optional;
mod m20251010_063315_enhance_sessions_security;
mod m20251011_024155_remove_closed_at_from_games;
mod m20251011_040630_games_max_tickets;
mod m20251011_061042_create_purchases_table_and_restructure;
mod m20251011_064042_make_purchase_serial_auto_increment;
mod m20251011_070011_add_missing_indexes;
mod m20251011_172622_closes_at_to_game;
mod m20251015_074524_email_optional;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251008_181629_create_users::Migration),
            Box::new(m20251008_190014_create_sessions::Migration),
            Box::new(m20251008_191518_create_games::Migration),
            Box::new(m20251008_191905_create_lotteries::Migration),
            Box::new(m20251008_193237_create_client_lotteries::Migration),
            Box::new(m20251008_193817_create_winners::Migration),
            Box::new(m20251008_195322_create_games_metadata::Migration),
            Box::new(m20251009_064229_mix_lotteries::Migration),
            Box::new(m20251009_080515_ticket_price_to_game::Migration),
            Box::new(m20251009_082241_closed_at_for_games::Migration),
            Box::new(m20251009_175344_numbers_datatype_128_bits::Migration),
            Box::new(m20251010_052043_finished_at_optional::Migration),
            Box::new(m20251010_063315_enhance_sessions_security::Migration),
            Box::new(m20251011_024155_remove_closed_at_from_games::Migration),
            Box::new(m20251011_040630_games_max_tickets::Migration),
            Box::new(m20251011_061042_create_purchases_table_and_restructure::Migration),
            Box::new(m20251011_064042_make_purchase_serial_auto_increment::Migration),
            Box::new(m20251011_070011_add_missing_indexes::Migration),
            Box::new(m20251011_172622_closes_at_to_game::Migration),
            Box::new(m20251015_074524_email_optional::Migration),
        ]
    }
}
