use thiserror::Error;
use crate::errors::db::sqlx_error::SqlxErrorWrapper;

pub enum ErrorWrapper {
    // A variant for your database errors
    Database(SqlxErrorWrapper),

    // A variant for authentication/authorization errors
    // Auth(AuthError),

    // You can also handle external library errors directly
    Redis(redis::RedisError),
}