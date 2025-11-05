use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::errors::{Error, ErrorKind};
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("The provided authentication token has expired.")]
    TokenExpired,
    #[error("The authentication token is malformed or invalid.")]
    InvalidTokenFormat,
    #[error("The authentication token signature is invalid.")]
    InvalidSignature,
    #[error("A JWT processing error occurred")]
    JwtError,
}

// Our custom mapping logic
impl From<Error> for JwtError {
    fn from(err: Error) -> Self {
        match err.kind() {
            ErrorKind::ExpiredSignature => JwtError::TokenExpired,
            ErrorKind::InvalidToken => JwtError::InvalidTokenFormat,
            ErrorKind::InvalidSignature => JwtError::InvalidSignature,
            _ => JwtError::JwtError,
        }
    }
}

impl IntoResponse for JwtError {
    fn into_response(self) -> Response {
        let res = match self {
            JwtError::TokenExpired => (StatusCode::UNAUTHORIZED, create_json_error_str!("Token expired")),
            JwtError::InvalidTokenFormat => (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR")),
            JwtError::InvalidSignature => (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR")),
            JwtError::JwtError => (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR")),
        };

        res.into_response()
    }
}