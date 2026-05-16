use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::db::soundcloud::{DeezerFullTrackResponse, SoundcloudFullTrackResponse};

pub enum IsUserExistsRes {
    NotExists,
    UsernameExists,
    EmailExists,
    EmailAndUsernameExists,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserTable {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SessionTable {
    pub id: Uuid,
    pub user_id: i64,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserPlaylist {
    pub id: i64,
    pub title: String,
    pub owner_id: i64,
    pub last_track_added_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TrackInUserPlaylist {
    pub id: i64,
    pub title: String,
    pub track_id: i64,
    pub platform_id: TrackPlatform,
    pub position: i32,
}

#[derive(Debug, Clone, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "track_platform")]
#[sqlx(rename_all = "lowercase")]
pub enum TrackPlatform {
    Deezer,
    Soundcloud,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlaylistInUser {
    pub id: i64,
    pub title: String,
    pub last_track_added_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserWithPlaylists {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub playlists: Vec<PlaylistInUser>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackInPlaylistResponse {
    pub track_in_playlist_id: i64,
    pub position: i32,
    pub platform: TrackPlatform,
    pub track_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct UserPlaylistWithTracks {
    pub id: i64,
    pub title: String,
    pub owner_id: i64,
    pub owner_name: String,
    pub last_track_added_at: Option<DateTime<Utc>>,
    pub tracks: Vec<TrackInPlaylistResponse>,
    pub found_tracks_soundcloud: Vec<SoundcloudFullTrackResponse>,
    pub found_tracks_deezer: Vec<DeezerFullTrackResponse>,
}
