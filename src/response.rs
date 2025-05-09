use axum::Json;
use axum::body::Body;
use axum::http::StatusCode;
use axum::http::header;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;
use tokio_util::io::ReaderStream;

#[derive(Default, Serialize)]
pub struct Empty;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub data: T,
    pub message: String,
}

impl<T: Serialize> ApiResponse<T> {
    /// success response
    pub fn success(data: T) -> Response<Body> {
        let resp = ApiResponse { code: StatusCode::OK.as_u16(), data, message: String::new() };
        (StatusCode::OK, Json(resp)).into_response()
    }
}

impl ApiResponse<Empty> {
    /// error response
    pub fn error<M: std::fmt::Display>(msg: M, status: StatusCode) -> Response<Body> {
        let resp = ApiResponse { code: status.as_u16(), data: Empty, message: msg.to_string() };
        (status, Json(resp)).into_response()
    }

    /// file stream response
    pub fn stream(stream: ReaderStream<tokio::fs::File>, filename: &str) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .header(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", filename))
            .body(Body::from_stream(stream))
            .unwrap()
            .into_response()
    }
}
