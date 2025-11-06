use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use lettre::{address::AddressError, transport::smtp::Error as SmtpError};
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Error, Debug)]
pub enum MailerError {
    #[error("Failed to build email: {0}")]
    EmailBuildError(#[from] lettre::error::Error),

    #[error("Failed to parse email address: {0}")]
    AddressParseError(#[from] AddressError),

    #[error("Failed to send email via SMTP: {0}")]
    SmtpTransportError(#[from] SmtpError),
}

impl IntoResponse for MailerError {
    fn into_response(self) -> Response {
        let res = match self {
            MailerError::AddressParseError(_) => (StatusCode::BAD_REQUEST, create_json_error_str!("Bad email address")),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("Captain we fucked up"))
        };

        res.into_response()
    }
}