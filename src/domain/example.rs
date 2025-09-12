use axum::Extension;
use axum::response::IntoResponse;

use crate::common::response::ApiResponse;

pub async fn handler(Extension(trace_id): Extension<String>) -> impl IntoResponse {
    tracing::info!("example handler");
    ApiResponse::success("example handler".to_string(), trace_id)
}
