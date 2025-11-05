use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Error, Debug)]
pub enum UserCreationError {
    #[error("Account with this email already exists")]
    EmailAlreadyExists,
    #[error("Unexpected")]
    Other,
}

impl IntoResponse for UserCreationError {
    fn into_response(self) -> Response {
        let res = match self {
            UserCreationError::EmailAlreadyExists => {
                (StatusCode::CONFLICT, create_json_error_str!("Email or username already exists"))
            }
            UserCreationError::Other => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR"))
            }
        };

        res.into_response()
    }
}