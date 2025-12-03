use crate::errors::music_services::deezer_api_error::DeezerApiError;

// Macro to bake strings in compile time
#[macro_export]
macro_rules! create_json_error_str {
    ($l:literal) => {
        concat!(r#"{"error":""#, $l, r#""}"#)
    };
}

pub fn create_json_error_str(error: String) -> String {
    format!(r#"{{"error":"{}"}}"#, error)
}