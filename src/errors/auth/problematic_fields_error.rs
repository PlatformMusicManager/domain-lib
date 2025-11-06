use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub struct ProblematicFieldsError (pub StatusCode, pub String);
impl IntoResponse for ProblematicFieldsError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}