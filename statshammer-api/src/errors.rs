use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;

// TODO: Custom Error Handler for all errors (e.g: Serialise issues)

#[derive(Serialize)]
struct ErrorBody {
    status_code: u16,
    message: String,
    phrase: String,
}

pub struct ApiError {
    status_code: StatusCode,
    message: String,
}
impl ApiError {
    pub fn new(status_code: StatusCode, message: &str) -> Self {
        Self {
            status_code,
            message: message.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorBody {
            status_code: self.status_code.into(),
            message: self.message,
            phrase: self
                .status_code
                .canonical_reason()
                .unwrap_or("<unknown status code>")
                .to_string(),
        };
        (self.status_code, Json(body)).into_response()
    }
}
