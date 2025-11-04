use chrono::{DateTime, Utc};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ToRedisArgs, FromRedisValue)]
pub struct UserAwaitVerification {
    pub email: String,
    pub password_hash: String,
    pub code: String,
    pub active_until: DateTime<Utc>
}