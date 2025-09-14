use std::time::Instant;

use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use tracing::info;
use tracing::info_span;
use tracing::warn;

use crate::common::response::ApiResponse;

#[inline]
pub fn new_trace_id() -> String {
    let mut b = [0u8; 16];
    fastrand::fill(&mut b);
    let mut s = String::with_capacity(32);
    for &x in &b {
        use std::fmt::Write;
        let _ = write!(&mut s, "{:02x}", x);
    }
    s
}

pub async fn logging<B>(mut req: Request<B>, next: Next) -> Result<Response, Response>
where
    B: Send + Into<axum::body::Body>,
{
    let start = Instant::now();
    let trace_id = new_trace_id();

    // 创建请求 span
    let method = req.method().clone();
    let uri = req.uri().clone();
    let span = info_span!("request", %trace_id, %method, %uri);
    let _guard = span.enter();

    // 把 trace_id 放进当前 span 的 extensions，供 IntoResponse/其他地方读取
    req.extensions_mut().insert(trace_id.clone());

    // 请求日志
    info!("request");

    let resp = next.run(req.map(Into::into)).await;

    // 处理响应
    let status = resp.status();
    let elapsed = start.elapsed();

    info!(elapsed_ms = %elapsed.as_millis(), "response");

    if status == StatusCode::OK {
        return Ok(resp);
    }

    // 非 200：统一包装为 ApiResponse，HTTP 状态码改为 200
    let resp = ApiResponse::<()>::failure(status.as_u16(), status.as_str(), trace_id);
    warn!(%status, "invalid response");

    Ok(resp.into_response())
}
