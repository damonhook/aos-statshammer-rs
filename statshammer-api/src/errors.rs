use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

// TODO: Possibly look at `message` and `details` fields
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

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        let status_code = match rejection {
            JsonRejection::JsonDataError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
            JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status_code,
            message: rejection.to_string(),
        }
    }
}

impl From<QueryRejection> for ApiError {
    fn from(rejection: QueryRejection) -> Self {
        Self {
            status_code: StatusCode::UNPROCESSABLE_ENTITY,
            message: rejection.to_string(),
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
        (self.status_code, axum::Json(body)).into_response()
    }
}

pub fn panic_handler(err: Box<dyn std::any::Any + Send + 'static>) -> Response {
    println!("{:?}", err);
    let message = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };
    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, &message).into_response()
}
