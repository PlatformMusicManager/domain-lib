use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::{ create_json_error_str };

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Session not found")]
    SessionNotFound,
    #[error("Session was updated")]
    SessionWasUpdated,
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        let res = match self {
            SessionError::SessionNotFound => (StatusCode::UNAUTHORIZED, create_json_error_str!("Session not found")),
            SessionError::SessionWasUpdated => (StatusCode::CONFLICT, create_json_error_str!("Session not found")),
        };

        res.into_response()
    }
}