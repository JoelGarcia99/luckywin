use sea_orm::{DbErr, SqlErr};

use crate::shared::errors::Rfc7807Problem;

/// Parses a given DB error into human readable string
pub fn process_db_err(err: &DbErr) -> Rfc7807Problem {
    // Check for check constraint violations in the raw error string
    let err_string = err.to_string();

    // Check for date/time field value out of range
    if err_string.contains("date/time field value out of range") {
        return Rfc7807Problem::simple_bad_request(
            None,
            Some("La hora debe estar entre 00:00 y 23:59"),
            Some("DATETIME_OUT_OF_RANGE"),
        );
    }

    if let Some(sql_err) = err.sql_err() {
        tracing::debug!("SQL error: {:?}", sql_err);
        return match sql_err {
            SqlErr::ForeignKeyConstraintViolation(error) => {
                Rfc7807Problem::simple_bad_request(
                    None,
                    Some(&error),
                    Some("FOREIGN_KEY_CONSTRAINT_VIOLATION"),
                )
            }
            SqlErr::UniqueConstraintViolation(error) => {
                Rfc7807Problem::simple_bad_request(
                    None,
                    Some(&error),
                    Some("UNIQUE_CONSTRAINT_VIOLATION"),
                )
            }
            _ => {
                Rfc7807Problem::server_error(
                    None,
                    Some(&err.to_string()),
                    Some("CRITICAL_DATABASE_ERROR"),
                )
            }
        };
    }

    // If no SQL error is present it's likely a critical server error so we
    // have to let the user know it so he can reach IT
    Rfc7807Problem::server_error(None, None, Some("CRITICAL_DATABASE_ERROR"))
}
