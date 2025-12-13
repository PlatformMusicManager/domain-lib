use axum::body::Bytes;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use thiserror::Error;
use url::ParseError;
use tokio::sync::broadcast::error::{SendError};
use crate::create_json_error_str;
use crate::errors::create_json_error_str::create_json_error_str;

#[derive(Error, Debug, Clone)]
pub enum BodyStreamError {
    #[error("An error occurred while producing the stream data")]
    SourceError,

    #[error("The broadcast stream receiver lagged and lost {0} messages. The stream is now corrupt.")]
    Lagged(u64),

    #[error("Fail to get next chunk from soundcloud")]
    ChunkError,
}

#[derive(Error, Debug)]
pub enum SoundcloudApiError {
    #[error("Invalid request to SoundCloud")]
    InvalidRequestToSoundcloud(#[from] reqwest::Error),

    #[error("Error while creating URL for SoundCloud request, invalid data was provided")]
    UrlParseError(#[from] ParseError),

    #[error("Error while deserialize")]
    DeserializeError(#[from] serde_json::Error),

    #[error("No data for track in response")]
    NoTrackDataInResponse,

    #[error("No media data attached in track in response")]
    NoMediaDataInResponse,

    #[error("Track data is not full")]
    TrackDataIsNotFull,

    #[error("Tx send error")]
    TxSendError(#[from] SendError<Result<Bytes, BodyStreamError>>),
}

impl IntoResponse for SoundcloudApiError{
    fn into_response(self) -> Response {
        let res = match self {
            SoundcloudApiError::InvalidRequestToSoundcloud(err) => {
                let status = err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                (status, create_json_error_str!("Soundcloud server response with error"))
            }
            SoundcloudApiError::UrlParseError(_) =>
                (
                    StatusCode::BAD_REQUEST,
                    create_json_error_str!("Fail to parse URL with provided params")
                ),
            SoundcloudApiError::DeserializeError(_) =>
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    create_json_error_str!("Deserialize soundcloud response error")
                ),
            SoundcloudApiError::NoTrackDataInResponse =>
                (
                    StatusCode::NOT_FOUND,
                    create_json_error_str!("No track data in response")
                ),
            SoundcloudApiError::NoMediaDataInResponse =>
                (
                    StatusCode::NOT_FOUND,
                    create_json_error_str!("No media data in response")
                ),
            SoundcloudApiError::TrackDataIsNotFull =>
                (
                    StatusCode::NOT_FOUND,
                    create_json_error_str!("TrackData is not full")
                ),
            SoundcloudApiError::TxSendError(_) =>
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    create_json_error_str!("Tx send error")
                )
        };

        res.into_response()
    }
}