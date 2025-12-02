use serde::{Deserialize, Serialize};
use crate::models::music_api::track::Track;

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub parent_user_id: String,
    pub parent_username: String,
    pub parent_picture: String,
    pub picture: String,
    pub tracks: Vec<Track>,
    pub size: u32,
}