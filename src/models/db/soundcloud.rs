// Types

use crate::models::music_api::artist::ApiArtist;
use crate::models::music_api::playlist::ApiPlaylist;
use crate::models::music_api::services::ApiServices;
use crate::models::music_api::services::ApiServices::Soundcloud;
use crate::models::music_api::track::ApiTrack;
use serde::{Deserialize, Serialize};
use url::Position;

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
    pub author_id: i64,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "playlist_input_soundcloud")]
pub struct PlaylistInputSoundcloud {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
    pub author_id: i64,
}

pub struct CreateReplacePlaylistInput {
    pub playlist: PlaylistInputSoundcloud,
    pub playlist_author: AuthorInputSoundcloud,
    pub tracks: Vec<TrackInputSoundcloud>,
    pub track_authors: Vec<AuthorInputSoundcloud>,
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
    pub author: AuthorTableSoundcloud,
}

impl TrackFromPlaylistResponse {
    pub fn into_with_platform(self, platform: ApiServices) -> ApiTrack {
        ApiTrack {
            id: self.id.to_string(),
            service: platform,
            title: self.title.to_string(),
            artists: vec![self.author.into()],
            picture: self.img,
            alb_id: None,
            alb_title: None,
            duration: self.duration,
            track_url: None,
            track_token: None,
            platform,
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

impl FullPlaylistResponse {
    #[inline(always)]
    fn into(self, platform: ApiServices) -> ApiPlaylist {
        ApiPlaylist {
            id: self.id.to_string(),
            title: self.title,
            parent_user_id: self.author.id.to_string(),
            parent_username: self.author.title,
            parent_picture: self.author.img,
            picture: self.img,
            size: self.tracks.len() as u32,
            tracks: self
                .tracks
                .into_iter()
                .map(|t| t.into_with_platform(platform))
                .collect(),
            platform,
        }
    }
}

#[derive(Deserialize)]
pub struct SoundcloudFullPlaylistResponse(pub FullPlaylistResponse);

impl Into<ApiPlaylist> for SoundcloudFullPlaylistResponse {
    fn into(self) -> ApiPlaylist {
        self.0.into(ApiServices::Soundcloud)
    }
}

#[derive(Deserialize)]
pub struct DeezerFullPlaylistResponse(pub FullPlaylistResponse);

impl Into<ApiPlaylist> for DeezerFullPlaylistResponse {
    fn into(self) -> ApiPlaylist {
        self.0.into(ApiServices::Deezer)
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

// Trait removed

#[derive(Deserialize)]
pub struct SoundcloudFullTrackResponse(pub FullTrackResponse);

impl From<SoundcloudFullTrackResponse> for ApiTrack {
    fn from(val: SoundcloudFullTrackResponse) -> Self {
        val.0.into(ApiServices::Soundcloud)
    }
}

#[derive(Deserialize)]
pub struct DeezerFullTrackResponse(pub FullTrackResponse);

impl From<DeezerFullTrackResponse> for ApiTrack {
    fn from(val: DeezerFullTrackResponse) -> Self {
        val.0.into(ApiServices::Deezer)
    }
}

impl FullTrackResponse {
    #[inline(always)]
    fn into(self, platform: ApiServices) -> ApiTrack {
        ApiTrack {
            id: self.id.to_string(),
            service: platform,
            title: self.title,
            artists: vec![self.author.into()],
            picture: self.img,
            platform,
            duration: self.duration,
            alb_id: None,
            alb_title: None,
            track_url: None,
            track_token: None,
        }
    }
}

pub struct FullTracksResponse<T: Into<ApiTrack>> {
    pub found: Vec<T>,
    pub not_found: Vec<i64>,
}
