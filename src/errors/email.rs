use std::env;
use lettre::{address::AddressError, transport::smtp::Error as SmtpError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MailerError {
    #[error("Failed to read environment variable: {0}")]
    EnvVarError(#[from] env::VarError),

    #[error("Failed to build email: {0}")]
    EmailBuildError(#[from] lettre::error::Error),

    #[error("Failed to parse email address: {0}")]
    AddressParseError(#[from] AddressError),

    #[error("Failed to send email via SMTP: {0}")]
    SmtpTransportError(#[from] SmtpError),
}
