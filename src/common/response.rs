use axum::response::IntoResponse;
use axum::response::Response;
use serde::Deserialize;
use serde::Serialize;

use crate::common::trace;

/// A standardized API response format.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
    pub trace_id: String,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Create a success response with default message "success".
    pub fn success(data: T) -> Self {
        Self {
            status: 200,
            message: "success".into(),
            data: Some(data),
            trace_id: trace::current_trace_id(),
        }
    }

    /// Create a failure response with no data.
    pub fn failure(status: u16, message: impl Into<String>) -> Self {
        Self { status, message: message.into(), data: None, trace_id: trace::current_trace_id() }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    #[inline]
    fn into_response(self) -> Response { axum::Json(self).into_response() }
}
