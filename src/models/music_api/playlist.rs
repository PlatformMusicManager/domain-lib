use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub parent_user_id: String,
    pub parent_username: String,
    pub parent_picture: String,
    pub picture: String,
    pub size: u32,
}