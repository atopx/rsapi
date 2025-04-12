mod user;

use axum::Extension;
use axum::response::IntoResponse;
pub use user::login;

use crate::jwt;
use crate::response::Response;

pub async fn claims(Extension(claims): Extension<jwt::Claims>) -> impl IntoResponse {
    Response::success(claims)
}
