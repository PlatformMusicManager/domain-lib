use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Debug, Error)]
pub enum SqlxErrorWrapper {
    #[error("Database error: {0}")]
    SqlxError(sqlx::Error),
}

// This allows us to use the `?` operator to automatically convert
impl From<sqlx::Error> for SqlxErrorWrapper {
    fn from(err: sqlx::Error) -> Self {
        SqlxErrorWrapper::SqlxError(err)
    }
}

impl IntoResponse for SqlxErrorWrapper {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            SqlxErrorWrapper::SqlxError(sqlx_error) => {
                // Now we can match on the specific sqlx::Error variants
                match sqlx_error {
                    sqlx::Error::RowNotFound => {
                        (StatusCode::NOT_FOUND, create_json_error_str!("The requested resource was not found."))
                    }
                    // You can add more specific sqlx error matches here
                    sqlx::Error::Database(err) if err.is_unique_violation() => {
                        (StatusCode::CONFLICT, create_json_error_str!("A record with this value already exists."))
                    }
                    _ => {
                        // For any other database error, return a generic 500
                        (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("An internal database error occurred."))
                    }
                }
            }
        };

        (status, error_message).into_response()
    }
}