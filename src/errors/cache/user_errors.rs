use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Fale to parse entity from db")]
    ParseError,
    #[error("User not found")]
    UserNotFound,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let res = match self {
            UserError::ParseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR"))
            },
            UserError::UserNotFound => {
                (StatusCode::NOT_FOUND, create_json_error_str!("User not found"))
            }
        };

        res.into_response()
    }
}