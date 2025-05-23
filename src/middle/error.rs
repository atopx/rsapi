use axum::body::to_bytes;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;

use crate::response;
use crate::response::Empty;

pub async fn error_handler(req: Request, next: Next) -> impl IntoResponse {
    // 执行后续中间件或处理函数
    let resp = next.run(req).await;

    // 如果状态码为200，直接返回响应
    if resp.status() == StatusCode::OK {
        return resp;
    }

    // 如果扩展中已存放了错误信息，则优先使用扩展中的信息返回
    if let Some(error) = resp.extensions().get::<String>() {
        return response::ApiResponse::<Empty>::error(error, resp.status());
    }

    // 消耗响应，将其拆分为 parts 和 body 部分
    let (parts, body) = resp.into_parts();

    // 构造自定义的错误响应返回
    let bytes = to_bytes(body, 4096).await.unwrap_or_default();
    let message = String::from_utf8_lossy(&bytes).to_string();
    response::ApiResponse::error(&message, parts.status)
}
