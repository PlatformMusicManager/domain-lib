// Types

use serde::{Deserialize, Serialize};
use crate::models::music_api::album::{ApiAlbum, ApiTrackInAlbum};
use crate::models::music_api::artist::ApiArtist;
use crate::models::music_api::services::ApiServices::Deezer;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "album_input_deezer")]
pub struct AlbumInputDeezer {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "track_input_deezer")]
pub struct TrackInputDeezer {
    pub id: i64,
    pub title: String,
    pub duration: i32,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "author_input_deezer")]
pub struct AuthorInputDeezer {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
}

// Entity's

#[derive(Debug, sqlx::FromRow)]
pub struct TrackTableDeezer {
    pub id: i64,
    pub title: String,
    pub duration: i32,
    pub img: Option<String>,
    pub album_id: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AlbumTableDeezer {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
    pub author_id: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AuthorTableDeezer {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
}


// RETURN JSONS

#[derive(Debug, Serialize, Deserialize)]
pub struct FullAlbumResponse {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
    pub author: AuthorInfo,
    pub tracks: Vec<TrackInfo>,
}

impl Into<ApiAlbum> for FullAlbumResponse {
    fn into(self) -> ApiAlbum {
        ApiAlbum {
            id: self.id.to_string(),
            title: self.title,
            img: self.img,
            artists: vec![self.author.into()],
            tracks: self.tracks.into_iter().map(|t| t.into()).collect(),
            service: Deezer,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorInfo {
    pub id: i64,
    pub name: String,
    pub img: Option<String>,
}

impl Into<ApiArtist> for AuthorInfo {
    fn into(self) -> ApiArtist {
        ApiArtist {
            id: self.id.to_string(),
            username: self.name,
            picture: self.img,
            is_dummy: false
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackInfo {
    pub id: i64,
    pub title: String,
    pub duration: i32,
    pub img: Option<String>,
}

impl Into<ApiTrackInAlbum> for TrackInfo {
    fn into(self) -> ApiTrackInAlbum {
        ApiTrackInAlbum {
            id: self.id.to_string(),
            title: self.title,
            duration: self.duration,
            track_url: None,
            track_token: None
        }
    }
}