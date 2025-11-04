use chrono::{DateTime, Utc};
use redis::ToRedisArgs;

#[derive(Debug)]
pub struct UserAwaitVerification {
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub attempts: u8,
}

impl UserAwaitVerification {
    pub fn to_hash_array(self) -> [(impl ToRedisArgs, impl ToRedisArgs); 6] {
        [
            ("email", self.email),
            ("username", self.username),
            ("password_hash", self.password_hash),
            ("code", self.code),
            ("created_at", self.created_at.to_rfc3339()),
            ("attempts", self.attempts.to_string()),
        ]
    }
}