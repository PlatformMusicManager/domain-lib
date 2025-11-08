use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use redis::RedisError;
use crate::create_json_error_str;
use crate::errors::auth::jwt_error::JwtError;
use crate::errors::auth::problematic_fields_error::ProblematicFieldsError;
use crate::errors::cache::session_errors::SessionError;
use crate::errors::cache::user_errors::UserError;
use crate::errors::cache::verify_user_errors::UserVerifyError;
use crate::errors::db::session::{SessionCreationError, SessionUpdateError};
use crate::errors::db::sqlx_error::SqlxErrorWrapper;
use crate::errors::db::user::UserCreationError;
use crate::errors::email::MailerError;
use password_hash::errors::Error as PasswordHashError;

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

    // auth
    ProblematicFieldsError(ProblematicFieldsError),
    FailedToParse,
    
    // mailer
    MailerError(MailerError),
    
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

impl From<ProblematicFieldsError> for AppError {
    fn from(err: ProblematicFieldsError) -> Self {
        AppError::ProblematicFieldsError(err)
    }
}

impl From<MailerError> for AppError {
        fn from(err: MailerError) -> Self {
            AppError::MailerError(err)
        }
}

impl From<PasswordHashError> for AppError {
    fn from(_err: PasswordHashError) -> Self {
        AppError::FailedToParse
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
            AppError::RedisError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR")).into_response()
            }
            AppError::ProblematicFieldsError(problematic_fields_error) => {
                problematic_fields_error.into_response()
            }
            AppError::MailerError(mailer_error) => {
                mailer_error.into_response()
            }
            AppError::FailedToParse => {
                (StatusCode::INTERNAL_SERVER_ERROR, create_json_error_str!("INTERNAL_SERVER_ERROR")).into_response()
            }
        }
    }
}