use axum::body::Body;
use axum::http::header;
use axum::http::StatusCode;
use axum::response;
use axum::response::IntoResponse;
use axum::Json;
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

impl<T> Response<T> {
    pub fn file(
        filename: String, body: Body, content_type: Option<String>,
    ) -> axum::http::Response<Body> {
        let content_type = content_type.unwrap_or("application/octet-stream".to_string());
        let headers = [
            (header::CONTENT_TYPE, &content_type),
            (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
        ];
        (headers, body).into_response()
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> response::Response { (StatusCode::OK, Json(self)).into_response() }
}
