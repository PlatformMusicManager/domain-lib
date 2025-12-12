// Types

use serde::{Deserialize, Serialize};
use url::Position;
use crate::models::music_api::artist::ApiArtist;
use crate::models::music_api::playlist::ApiPlaylist;
use crate::models::music_api::services::ApiServices;
use crate::models::music_api::services::ApiServices::Soundcloud;
use crate::models::music_api::track::ApiTrack;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "author_input_soundcloud")]
pub struct AuthorInputSoundcloud {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "track_input_soundcloud")]
pub struct TrackInputSoundcloud {
    pub id: i64,
    pub title: String,
    pub duration: i64,
    pub img: Option<String>,
    pub author_id: i64
}


#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "playlist_input_soundcloud")]
pub struct PlaylistInputSoundcloud {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
    pub author_id: i64
}

pub struct CreateReplacePlaylistInput {
    pub playlist: PlaylistInputSoundcloud,
    pub playlist_author: AuthorInputSoundcloud,
    pub tracks: Vec<TrackInputSoundcloud>,
    pub track_authors: Vec<AuthorInputSoundcloud>
}

// Table

#[derive(Debug, sqlx::FromRow, Deserialize)]
pub struct TrackTableSoundcloud {
    pub id: i64,
    pub title: String,
    pub duration: i32,
    pub img: Option<String>,
    pub author_id: Option<i32>,
}

#[derive(Debug, sqlx::FromRow, Deserialize)]
pub struct AuthorTableSoundcloud {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
}

impl Into<ApiArtist> for AuthorTableSoundcloud {
    fn into(self) -> ApiArtist {
        ApiArtist {
            id: self.id.to_string(),
            username: self.title,
            picture: self.img,
            is_dummy: false,
        }
    }
}

// Return json struct's

#[derive(Deserialize)]
pub struct TrackFromPlaylistResponse {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
    pub duration: i64,
    pub position: u32,
    pub author: AuthorTableSoundcloud
}

impl Into<ApiTrack> for TrackFromPlaylistResponse {
    fn into(self) -> ApiTrack {
        ApiTrack {
            id: self.id.to_string(),
            service: Soundcloud,
            title: self.title.to_string(),
            artists: vec![self.author.into()],
            alb_id: None,
            alb_title: None,
            duration: self.duration,
            track_url: None,
            track_token: None,
        }
    }
}


#[derive(Deserialize)]
pub struct FullPlaylistResponse {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
    pub playlist_size: u32,
    pub author: AuthorTableSoundcloud,
    pub tracks: Vec<TrackFromPlaylistResponse>,
}

impl Into<ApiPlaylist> for FullPlaylistResponse {
    fn into(self) -> ApiPlaylist {
        ApiPlaylist {
            id: self.id.to_string(),
            title: self.title,
            parent_user_id: self.author.id.to_string(),
            parent_username: self.author.title,
            parent_picture: self.author.img,
            picture: self.img,
            size: self.tracks.len() as u32,
            tracks: self.tracks.into_iter().map(|t| t.into()).collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct FullTrackResponse {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
    pub author: AuthorTableSoundcloud,
    pub duration: i64,
}

impl Into<ApiTrack> for FullTrackResponse {
    fn into(self) -> ApiTrack {
        ApiTrack {
            id: self.id.to_string(),
            service: ApiServices::Deezer,
            title: self.title,
            artists: vec![self.author.into()],
            duration: self.duration,
            alb_id: None,
            alb_title: None,
            track_url: None,
            track_token: None,
        }
    }
}

pub struct FullTracksResponse {
    pub found: Vec<FullTrackResponse>,
    pub not_found: Vec<i64>
}