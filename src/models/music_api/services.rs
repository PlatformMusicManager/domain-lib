use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum ApiServices {
    Deezer,
    Soundcloud
}