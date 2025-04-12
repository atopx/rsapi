use axum::Json;
use axum::http::StatusCode;
use axum::response;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    pub data: T,
    pub code: u16,
    pub message: String,
}

#[derive(Debug, Serialize, Default)]
pub struct Empty;

impl<T: Serialize + Default> Response<T> {
    pub fn new(data: T, message: String, status: StatusCode) -> Self {
        Self { data, message, code: status.as_u16() }
    }
    pub fn error(message: &str, status: StatusCode) -> Self {
        Response::new(T::default(), message.to_string(), status)
    }

    pub fn success(data: T) -> Self {
        let code = StatusCode::OK;
        Self::new(data, code.to_string(), code)
    }
}

impl<T: Serialize> response::IntoResponse for Response<T> {
    fn into_response(self) -> response::Response { (StatusCode::OK, Json(self)).into_response() }
}
