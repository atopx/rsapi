use axum::body::Body;
use axum::body::to_bytes;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;

use crate::response;
use crate::response::Empty;

pub async fn error_handler(req: Request, next: Next) -> Result<Response<Body>, Response<Body>> {
    // 执行后续中间件或处理函数
    let resp = next.run(req).await;

    // 如果状态码为200，直接返回响应
    if resp.status() == StatusCode::OK {
        return Ok(resp);
    }

    // 如果扩展中已存放了错误信息，则优先使用扩展中的信息返回
    if let Some(error) = resp.extensions().get::<String>() {
        return Err(response::Response::<Empty>::error(error, resp.status()).into_response());
    }

    // 消耗响应，将其拆分为 parts 和 body 部分
    let (parts, body) = resp.into_parts();
    // 使用 hyper::body::to_bytes 将 body 转为字节流
    let bytes = to_bytes(body, 4096).await.unwrap_or_default();
    // 将字节流转换为字符串，如果不是合法的 UTF-8 则返回默认信息
    let message = String::from_utf8_lossy(&bytes).to_string();

    // 构造自定义的错误响应返回
    Err(response::Response::<Empty>::error(&message, parts.status).into_response())
}
