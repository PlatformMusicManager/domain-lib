use axum::http::StatusCode;
use password_hash::errors::Error as PasswordHashError;
use redis::RedisError;
use thiserror::Error;

use crate::errors::auth::cookie_error::CookieErrors;
use crate::errors::auth::jwt_error::JwtError;
use crate::errors::auth::problematic_fields_error::ProblematicFieldsError;
use crate::errors::cache::session_errors::SessionError;
use crate::errors::cache::user_errors::UserError;
use crate::errors::cache::verify_user_errors::UserVerifyError;
use crate::errors::db::session::{SessionCreationError, SessionUpdateError};
use crate::errors::db::sqlx_error::SqlxErrorWrapper;
use crate::errors::db::user::UserCreationError;
use crate::errors::email::MailerError;
use crate::errors::music_services::deezer_api_error::DeezerApiError;
use crate::errors::music_services::soundcloud_api_error::SoundcloudApiError;
use crate::{create_json_error_str, define_app_error};

define_app_error! {
    // --- Part 1: Enum Definition ---
    #[derive(Debug, Error)]
    pub enum AppError {
        // DB
        #[error(transparent)]
        SessionCreationError(#[from] SessionCreationError),
        #[error(transparent)]
        SessionUpdateError(#[from] SessionUpdateError),
        #[error(transparent)]
        UserCreationError(#[from] UserCreationError),
        #[error(transparent)]
        SqlxError(#[from] SqlxErrorWrapper),

        // cache
        #[error(transparent)]
        SessionError(#[from] SessionError),
        #[error(transparent)]
        UserError(#[from] UserError),
        #[error(transparent)]
        UserVerifyError(#[from] UserVerifyError),

        // Jwt
        #[error(transparent)]
        JwtError(#[from] JwtError),
        #[error(transparent)]
        RedisError(#[from] RedisError),

        // auth
        #[error(transparent)]
        ProblematicFieldsError(#[from] ProblematicFieldsError),
        #[error("Fail to decrypt/encrypt password: {0}")]
        FailedToParse(PasswordHashError),

        // mailer
        #[error(transparent)]
        MailerError(#[from] MailerError),

        // Cookie
        #[error(transparent)]
        CookieError(#[from] CookieErrors),

        #[error(transparent)]
        DeezerApiError(#[from] DeezerApiError),

        #[error(transparent)]
        SoundclodApiError(#[from] SoundcloudApiError)
    }

    // --- Part 2: Response Logic ---
    response: {
        delegates: [
            SessionCreationError,
            SessionUpdateError,
            UserCreationError,
            SqlxError,
            SessionError,
            UserError,
            UserVerifyError,
            JwtError,
            ProblematicFieldsError,
            MailerError,
            CookieError,
            DeezerApiError,
            SoundclodApiError
        ],
        custom: {
            AppError::RedisError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR")).into_response()
            },
            AppError::FailedToParse(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR")).into_response()
            }
        }
    }
}

// The separate `From` implementation remains as it is.
impl From<PasswordHashError> for AppError {
    fn from(err: PasswordHashError) -> Self {
        Self::FailedToParse(err)
    }
}
