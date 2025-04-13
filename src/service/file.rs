use axum::{
    Extension,
    extract::{Multipart, Path},
    response::IntoResponse,
};
use tracing::info;

use crate::{
    jwt,
    response::{Empty, Response},
};

/// example upload file
pub async fn upload(
    Extension(claims): Extension<jwt::Claims>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    Response::success(Empty)
}

/// example download file
pub async fn download(
    Path(filename): Path<String>,
    Extension(claims): Extension<jwt::Claims>,
) -> impl IntoResponse {
    info!(filename, "download");
    Response::success(Empty)
}
