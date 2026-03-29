use crate::models::music_api::{services::ApiServices, track::ApiTrack};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiPlaylist {
    pub id: String,
    pub title: String,
    pub parent_user_id: String,
    pub parent_username: String,
    pub parent_picture: Option<String>,
    pub platform: ApiServices,
    pub picture: Option<String>,
    pub tracks: Vec<ApiTrack>,
    pub size: u32,
}
