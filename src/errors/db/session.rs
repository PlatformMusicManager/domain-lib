use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use lettre::message::IntoBody;
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Error, Debug)]
pub enum SessionCreationError {
    #[error("Id already exists")]
    IdAlreadyExists,
    #[error("User not found")]
    UserNotFound,
}

impl IntoResponse for SessionCreationError {
    fn into_response(self) -> Response {
        let res = match self {
            SessionCreationError::IdAlreadyExists => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("Session ID already exists"))
            },
            SessionCreationError::UserNotFound => {
                (StatusCode::NOT_FOUND, create_json_error_str!("User not found"))
            }
        };

        res.into_response()
    }
}

#[derive(Error, Debug)]
pub enum SessionUpdateError {
    #[error("Session not found")]
    NotFound,
    #[error("Old token")]
    InvalidSerialNumber,
    #[error("Session expired")]
    Expired,
}

impl IntoResponse for SessionUpdateError {
    fn into_response(self) -> Response {
        let res = match self {
            SessionUpdateError::NotFound => {
                (StatusCode::BAD_REQUEST, create_json_error_str!("Session not found"))
            },
            SessionUpdateError::InvalidSerialNumber => {
                (StatusCode::NOT_FOUND, create_json_error_str!("User not found"))
            }
            SessionUpdateError::Expired => {
                (StatusCode::CONFLICT, create_json_error_str!("Session expired"))
            }
        };

        res.into_response()
    }
}