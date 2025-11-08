use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::create_json_error_str;

#[derive(Error, Debug)]
pub enum CookieErrors {
    #[error("Verification token not found")]
    VerificationTokenNotFound,
    #[error("Access token not found")]
    AccessTokenNotFound,
    #[error("Refresh token not found")]
    RefreshTokenNotFound,
}


impl IntoResponse for CookieErrors {
    fn into_response(self) -> Response {
        let res = match self {
            CookieErrors::VerificationTokenNotFound => {
                (
                    StatusCode::NOT_FOUND,
                    create_json_error_str!("Verification token not found, possible verification time ended")
                )
            },
            CookieErrors::AccessTokenNotFound => {
                (
                    StatusCode::UNAUTHORIZED,
                    create_json_error_str!("Access token possibly expired")
                )
            }
            CookieErrors::RefreshTokenNotFound => {
                (
                    StatusCode::UNAUTHORIZED,
                    create_json_error_str!("Refresh token not found, try to re-login")
                )
            }
        };
        res.into_response()
    }
}