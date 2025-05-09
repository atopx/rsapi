mod file;
mod user;

use axum::Extension;
use axum::response::IntoResponse;
pub use file::download;
pub use file::form_upload;
pub use file::upload;
pub use user::login;

use crate::jwt;
use crate::response::ApiResponse;

/// example need authorization api
pub async fn claims(Extension(claims): Extension<jwt::Claims>) -> impl IntoResponse { ApiResponse::success(claims) }
