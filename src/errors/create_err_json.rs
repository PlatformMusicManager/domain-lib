use axum::Json;
use serde_json::{json, Value};

#[macro_export]
macro_rules! create_json_error_str {
    ($l:literal) => {
        concat!(r#"{"error":""#, $l, r#""}"#)
    };
}