use serde::{Deserialize, Serialize};
use crate::models::music_api::artist::Artist;
use crate::models::music_api::services::Services;

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub service: Services,
    pub title: String,
    pub artists: Vec<Artist>,
    pub alb_id: Option<String>,
    pub alb_title: Option<String>,
    pub duration: u32,
    pub track_url: Option<String>,
    pub track_token: Option<String>,
}