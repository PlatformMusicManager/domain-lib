// Types

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
    pub duration: i32,
    pub img: Option<String>,
}

// Table

#[derive(Debug, sqlx::FromRow)]
pub struct TrackTableSoundcloud {
    pub id: i64,
    pub title: String,
    pub duration: i32,
    pub img: Option<String>,
    pub author_id: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AuthorTableSoundcloud {
    pub id: i64,
    pub title: String,
    pub img: Option<String>,
}
