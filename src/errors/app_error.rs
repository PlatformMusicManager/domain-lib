use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use redis::RedisError;
use thiserror::Error;
use crate::create_json_error_str;
use crate::errors::auth::jwt_error::JwtError;
use crate::errors::cache::session_errors::SessionError;
use crate::errors::cache::user_errors::UserError;
use crate::errors::cache::verify_user_errors::UserVerifyError;
use crate::errors::db::session::{SessionCreationError, SessionUpdateError};
use crate::errors::db::sqlx_error::SqlxErrorWrapper;
use crate::errors::db::user::UserCreationError;

#[derive(Debug)]
pub enum AppError {
    // DB
    SessionCreationError(SessionCreationError),
    SessionUpdateError(SessionUpdateError),
    UserCreationError(UserCreationError),
    SqlxError(SqlxErrorWrapper),

    // cache
    SessionError(SessionError),
    UserError(UserError),
    UserVerifyError(UserVerifyError),

    // Jwt
    JwtError(JwtError),
    RedisError(RedisError),
}

impl From <RedisError> for AppError {
    fn from (err: RedisError) -> Self {
        AppError::RedisError(err)
    }
}

impl From<SessionCreationError> for AppError {
    fn from(err: SessionCreationError) -> Self {
        AppError::SessionCreationError(err)
    }
}

impl From<SessionUpdateError> for AppError {
    fn from(err: SessionUpdateError) -> Self {
        AppError::SessionUpdateError(err)
    }
}

impl From<UserCreationError> for AppError {
    fn from(err: UserCreationError) -> Self {
        AppError::UserCreationError(err)
    }
}

impl From<SqlxErrorWrapper> for AppError {
    fn from(err: SqlxErrorWrapper) -> Self {
        AppError::SqlxError(err)
    }
}

impl From<JwtError> for AppError {
    fn from(err: JwtError) -> Self {
        AppError::JwtError(err)
    }
}



impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Match on the enum variant
        match self {
            // If it's a SessionError, get the inner error...
            AppError::SessionCreationError(session_error) => {
                // ...and call *its* into_response() method.
                session_error.into_response()
            }
            AppError::SessionUpdateError(session_error) => {
                // ...and call *its* into_response() method.
                session_error.into_response()
            }
            // If it's a UserError, get the inner error...
            AppError::UserCreationError(user_error) => {
                // ...and call *its* into_response() method.
                user_error.into_response()
            }
            // If it's a SqlxError, get the inner error...
            AppError::SqlxError(sqlx_error) => {
                // ...and call *its* into_response() method.
                sqlx_error.into_response()
            }
            AppError::UserError(user_error) => {
                user_error.into_response()
            }
            AppError::UserVerifyError(user_error) => {
                user_error.into_response()
            }
            AppError::JwtError(jwt_error) => {
                jwt_error.into_response()
            },
            AppError::SessionError(session_error) => {
                session_error.into_response()
            }
            AppError::RedisError(redis_error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR")).into_response()
            }

        }
    }
}