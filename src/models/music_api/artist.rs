use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Artist {
    pub id: String,
    pub username: String,
    pub picture: String,
    pub is_dummy: bool,
}