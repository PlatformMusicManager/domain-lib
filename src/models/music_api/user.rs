use serde::{Serialize};

#[derive(Serialize)]
pub struct ApiUser {
    pub id: String,
    pub img: String,
    pub username: String,
}
