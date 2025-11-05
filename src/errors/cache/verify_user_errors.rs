use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Error, Debug)]
pub enum UserVerifyError {
    #[error("Fale to parse entity from db")]
    ParseError,
    #[error("User not found, ")]
    UserNotFound,
    #[error("Exceeded amount of attempts")]
    ExceededAttempts,
    #[error("Wrong code")]
    WrongCode,
    #[error("Expired")]
    Expired,
}

impl IntoResponse for UserVerifyError {
    fn into_response(self) -> Response {
        let res = match self {
            UserVerifyError::ParseError => (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR")),
            UserVerifyError::UserNotFound | UserVerifyError::Expired => (StatusCode::NOT_FOUND, create_json_error_str!("NOT_FOUND")),
            UserVerifyError::ExceededAttempts => (StatusCode::NOT_ACCEPTABLE, create_json_error_str!("Exceeded attempts")),
            UserVerifyError::WrongCode => (StatusCode::FORBIDDEN, create_json_error_str!("Wrong code")),
        };

        res.into_response()
    }
}