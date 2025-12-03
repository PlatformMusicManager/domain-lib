use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ApiArtist {
    pub id: String,
    pub username: String,
    pub picture: Option<String>,
    pub is_dummy: bool,
}