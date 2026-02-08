use std::time::Instant;

use axum::http::HeaderMap;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use opentelemetry::global;
use opentelemetry::propagation::Extractor;
use opentelemetry::trace::TraceContextExt as _;
use tracing::Instrument as _;
use tracing::field;
use tracing::info;
use tracing::info_span;
use tracing::warn;
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

use crate::common::response::ApiResponse;

struct HeaderExtractor<'a>(&'a HeaderMap);

impl Extractor for HeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> { self.0.get(key).and_then(|value| value.to_str().ok()) }

    fn keys(&self) -> Vec<&str> { self.0.keys().map(|key| key.as_str()).collect() }
}

#[inline]
fn extract_remote_context(headers: &HeaderMap) -> opentelemetry::Context {
    global::get_text_map_propagator(|propagator| propagator.extract(&HeaderExtractor(headers)))
}

#[inline]
pub fn current_trace_id() -> String {
    let context = tracing::Span::current().context();
    context.span().span_context().trace_id().to_string()
}

pub async fn logging<B>(req: Request<B>, next: Next) -> Result<Response, Response>
where
    B: Send + Into<axum::body::Body>,
{
    let start = Instant::now();

    // 创建请求 span
    let span = info_span!(
        "request",
        trace_id = field::Empty,
        span_id = field::Empty,
        method = %req.method(),
        uri = %req.uri(),
        status_code = field::Empty
    );

    if let Some(remote_addr) = req.headers().get("REMOTE_ADDR").and_then(|value| value.to_str().ok()) {
        span.record("remote_addr", field::display(remote_addr));
    }

    // 继承上游 traceparent（如果存在），保持跨服务 trace 连续。
    let parent_context = extract_remote_context(req.headers());
    let _ = span.set_parent(parent_context);

    let context = span.context();

    span.record("trace_id", field::display(context.span().span_context().trace_id()));
    span.record("span_id", field::display(context.span().span_context().span_id()));

    // 请求日志
    info!(parent: &span, "request.start");

    let mut resp = next.run(req.map(Into::into)).instrument(span.clone()).await;

    // 处理响应
    let status = resp.status();
    let elapsed = start.elapsed();
    span.record("status_code", field::display(status.as_u16()));

    info!(parent: &span, elapsed = elapsed.as_millis(), "request.finish");

    // 非 200：统一包装为 ApiResponse，HTTP 状态码改为 200
    if status != StatusCode::OK {
        resp = ApiResponse::<()>::failure(status.as_u16(), status.as_str()).into_response();
        warn!(parent: &span, %status, "request.failure");
    }

    Ok(resp)
}
