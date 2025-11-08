use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Error, Debug)]
pub enum CookiesError {
    #[error("Verification token not found")]
    VerificationTokenNotFound,
    #[error("Access token not found")]
    AccessTokenNotFound,
    #[error("Refresh token not found")]
    RefreshTokenNotFound,
}


impl IntoResponse for CookiesError {
    fn into_response(self) -> Response {
        let res = match self {
            CookiesError::VerificationTokenNotFound => {
                (
                    StatusCode::NOT_FOUND,
                    create_json_error_str!("Verification token not found, possible verification time ended")
                )
            },
            CookiesError::AccessTokenNotFound => {
                (
                    StatusCode::UNAUTHORIZED,
                    create_json_error_str!("Access token possibly expired")
                )
            }
            CookiesError::RefreshTokenNotFound => {
                (
                    StatusCode::UNAUTHORIZED,
                    create_json_error_str!("Refresh token not found, try to re-login")
                )
            }
        };
        res.into_response()
    }
}