use serde::{Deserialize, Serialize};
use crate::models::music_api::album::Album;
use crate::models::music_api::artist::Artist;
use crate::models::music_api::playlist::Playlist;
use crate::models::music_api::track::Track;
use crate::models::music_api::user::User;

#[derive(Deserialize, Serialize)]
pub struct SearchPage {
    pub artists: Vec<Artist>,
    pub albums: Vec<Album>,
    pub tracks: Vec<Track>,
    pub playlists: Vec<Playlist>,
    pub users: Vec<User>,
}