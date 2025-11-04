use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum IsUserExistsRes {
    NotExists,
    UsernameExists,
    EmailExists,
    EmailAndUsernameExists,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserTable {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SessionTable {
    pub id: Uuid,
    pub user_id: i64,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserPlaylist {
    pub id: i64,
    pub title: String,
    pub owner_id: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TrackInUserPlaylist {
    pub id: i64,
    pub title: String,
    pub track_id: i64,
    pub platform_id: TrackPlatfrom,
    pub position: i32,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "track_platform")]
#[sqlx(rename_all = "lowercase")]
pub enum TrackPlatfrom {
    Deezer,
    Soundсloud,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlaylistInUser {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserWithPlaylists {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub playlists: Vec<PlaylistInUser>,
}
