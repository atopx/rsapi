mod file;
mod user;

use axum::Extension;
use axum::response::IntoResponse;
pub use file::download;
pub use file::upload;
pub use user::login;

use crate::jwt;
use crate::response::Response;

/// example need authorization api
pub async fn claims(Extension(claims): Extension<jwt::Claims>) -> impl IntoResponse {
    Response::success(claims)
}
