use axum::Extension;
use axum::extract::Multipart;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use tracing::info;

use crate::buffer::BufferBody;
use crate::jwt;
use crate::response::ApiResponse;
use crate::response::Empty;

/// example form upload file
pub async fn form_upload(Extension(claims): Extension<jwt::Claims>, mut multipart: Multipart) -> impl IntoResponse {
    ApiResponse::success(Empty)
}

/// example download file
pub async fn download(Path(filename): Path<String>, Extension(_claims): Extension<jwt::Claims>) -> impl IntoResponse {
    if filename.is_empty() {
        return ApiResponse::error("filename is empty", StatusCode::BAD_REQUEST);
    }
    match tokio::fs::File::open(&filename).await {
        Ok(file) => {
            let stream = tokio_util::io::ReaderStream::new(file);
            ApiResponse::stream(stream, &filename)
        }
        Err(e) => ApiResponse::error(e, StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct Params {
    pub action: String,
}

/// example body binary upload
pub async fn upload(
    Path(filename): Path<String>, Query(params): Query<Params>, BufferBody(body): BufferBody,
) -> impl IntoResponse {
    if params.action.is_empty() {
        return ApiResponse::error("invalid action", StatusCode::BAD_REQUEST);
    }
    if filename.is_empty() {
        return ApiResponse::error("filename is empty", StatusCode::BAD_REQUEST);
    }
    info!(filename, params.action, "upload");
    match std::fs::write(filename, body) {
        Ok(_) => ApiResponse::success(Empty),
        Err(_) => ApiResponse::error("write file error", StatusCode::INTERNAL_SERVER_ERROR),
    }
}
