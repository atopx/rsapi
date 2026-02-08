use axum::response::IntoResponse;

use crate::common::response::ApiResponse;

#[tracing::instrument(name="example-handler")]
pub async fn handler() -> impl IntoResponse {
    tracing::info!("example handler");
    ApiResponse::success("example handler".to_string())
}
