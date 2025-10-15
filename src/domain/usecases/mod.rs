pub mod close_game;
pub mod create_game;
pub mod create_purchase;
pub mod get_batch_by_ids;
pub mod get_current_user;
pub mod get_last_active_game;
pub mod get_lotteries_by_game;
pub mod get_purchases_by_game;
pub mod get_user_by_cellphone;
pub mod login;
pub mod logout;
// Deprecated: use mark_purchase_as_paid instead
// pub mod mark_lottery_as_paid;
pub mod mark_purchase_as_paid;
pub mod mark_purchase_as_unpaid;
// Deprecated: use create_purchase instead
// pub mod purchase_lottery;
pub mod refresh_token;
pub mod reopen_game;
pub mod update_current_user;
pub mod upsert_client;
