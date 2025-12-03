use serde::{Deserialize, Serialize};
use crate::models::music_api::artist::ApiArtist;
use crate::models::music_api::services::ApiServices;

#[derive(Serialize)]
pub struct ApiAlbum {
    pub id: String,
    pub img: Option<String>,
    pub title: String,
    pub service: ApiServices,
    pub artists: Vec<ApiArtist>,
    pub tracks: Vec<ApiTrackInAlbum>,
}

#[derive(Serialize)]
pub struct ApiTrackInAlbum {
    pub id: String,
    pub title: String,
    pub duration: i32,
    pub track_url: Option<String>,
    pub track_token: Option<String>,
}