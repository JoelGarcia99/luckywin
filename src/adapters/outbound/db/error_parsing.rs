use sea_orm::{DbErr, SqlErr};

use crate::shared::errors::Rfc7807Problem;

/// Parses a given DB error into human readable string
pub fn process_db_err(err: &DbErr) -> Box<Rfc7807Problem> {
    // Check for check constraint violations in the raw error string
    let err_string = err.to_string();

    // Check for date/time field value out of range
    if err_string.contains("date/time field value out of range") {
        return Box::new(Rfc7807Problem::simple_bad_request(
            None,
            Some("La hora debe estar entre 00:00 y 23:59"),
            Some("DATETIME_OUT_OF_RANGE"),
        ));
    }

    // Check for cellphone unique constraint violation
    if err_string.contains("idx_users_cellphone_deleted_at") {
        return Box::new(Rfc7807Problem::simple_bad_request(
            Some("Número de celular duplicado"),
            Some("¡Oops! ¡El número de celular ingresado ya está siendo usado por otra persona!"),
            Some("CELLPHONE_ALREADY_EXISTS"),
        ));
    }

    if let Some(sql_err) = err.sql_err() {
        tracing::debug!("SQL error: {:?}", sql_err);
        return match sql_err {
            SqlErr::ForeignKeyConstraintViolation(error) => {
                Box::new(Rfc7807Problem::simple_bad_request(
                    None,
                    Some(&error),
                    Some("FOREIGN_KEY_CONSTRAINT_VIOLATION"),
                ))
            }
            SqlErr::UniqueConstraintViolation(error) => {
                Box::new(Rfc7807Problem::simple_bad_request(
                    None,
                    Some(&error),
                    Some("UNIQUE_CONSTRAINT_VIOLATION"),
                ))
            }
            _ => {
                Box::new(Rfc7807Problem::server_error(
                    None,
                    Some(&err.to_string()),
                    Some("CRITICAL_DATABASE_ERROR"),
                ))
            }
        };
    }

    // If no SQL error is present it's likely a critical server error so we
    // have to let the user know it so he can reach IT
    Box::new(Rfc7807Problem::server_error(None, None, Some("CRITICAL_DATABASE_ERROR")))
}
