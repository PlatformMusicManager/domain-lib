use crate::models::music_api::artist::ApiArtist;
use crate::models::music_api::services::ApiServices;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ApiTrack {
    pub id: String,
    pub service: ApiServices,
    pub title: String,
    pub artists: Vec<ApiArtist>,
    pub alb_id: Option<String>,
    pub alb_title: Option<String>,
    pub duration: i64,
    pub picture: Option<String>,
    pub track_url: Option<String>,
    pub track_token: Option<String>,
}
