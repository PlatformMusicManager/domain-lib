use serde::{Deserialize, Serialize};
use crate::models::music_api::artist::Artist;
use crate::models::music_api::track::Track;

#[derive(Deserialize, Serialize)]
pub struct Album {
    pub id: String,
    pub img: String,
    pub title: String,
    pub artists: Vec<Artist>,
    pub tracks: Vec<Track>,
}