use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct ProblematicFieldsError (pub StatusCode, pub String);

impl Display for ProblematicFieldsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProblematicFieldsError: {}", self.1)
    }
}

impl IntoResponse for ProblematicFieldsError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}