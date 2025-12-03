use serde::{Deserialize, Serialize};
use crate::models::music_api::album::ApiAlbum;
use crate::models::music_api::artist::ApiArtist;
use crate::models::music_api::playlist::ApiPlaylist;
use crate::models::music_api::track::ApiTrack;
use crate::models::music_api::user::ApiUser;

#[derive(Serialize)]
pub struct ApiSearchPage {
    pub artists: Vec<ApiArtist>,
    pub albums: Vec<ApiAlbum>,
    pub tracks: Vec<ApiTrack>,
    pub playlists: Vec<ApiPlaylist>,
    pub users: Vec<ApiUser>,
}