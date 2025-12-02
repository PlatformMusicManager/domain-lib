use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Services {
    Deezer,
    Soundcloud
}