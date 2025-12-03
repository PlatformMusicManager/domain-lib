use std::num::ParseIntError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::Value;
use thiserror::Error;
use crate::create_json_error_str;
use crate::errors::create_json_error_str::create_json_error_str;

#[derive(Debug, Error)]
pub enum DeezerApiError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Failed to parse json: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("API returned an error: {0}")]
    ApiError(Value), // For generic API errors

    #[error("A valid API token is required")]
    TokenRequired(String), // A specific, typed error for our retry logic

    #[error("No content length on stream")]
    NoContentLength,

    #[error("Fail to parse id (expected digits)")]
    ParseIdError(String),

    #[error("Fail to parse duration expected i32 digit's")]
    ParseDurationError(String),

    #[error("No tracks found")]
    NoTracks,

    #[error("Fail to find author by art_id in authors")]
    AuthorNotFound,
}

impl IntoResponse for DeezerApiError {
    fn into_response(self) -> axum::response::Response {
        let res = match self {
            DeezerApiError::RequestError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("request failed"))
            }
            DeezerApiError::UrlParseError(_) => {
                (StatusCode::BAD_REQUEST, create_json_error_str!("URL parsing failed"))
            }
            DeezerApiError::JsonParseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("JSON parsing failed"))
            }
            DeezerApiError::ApiError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("API returned an error"))
            }
            DeezerApiError::TokenRequired(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("token required"))
            }
            DeezerApiError::NoContentLength => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("no content length"))
            }
            DeezerApiError::ParseIdError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("deezer parse id error"))
            }
            DeezerApiError::ParseDurationError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("parse duration error"))
            }
            DeezerApiError::NoTracks => {
                (StatusCode::BAD_REQUEST, create_json_error_str!("no tracks found"))
            }
            DeezerApiError::AuthorNotFound => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("author not found"))
            }
        };

        res.into_response()
    }
}
