use serde::{Deserialize, Serialize};
use crate::models::music_api::artist::ApiArtist;
use crate::models::music_api::services::ApiServices;

#[derive(Serialize)]
pub struct ApiTrack {
    pub id: String,
    pub service: ApiServices,
    pub title: String,
    pub artists: Vec<ApiArtist>,
    pub alb_id: Option<String>,
    pub alb_title: Option<String>,
    pub duration: i64,
    pub track_url: Option<String>,
    pub track_token: Option<String>,
}