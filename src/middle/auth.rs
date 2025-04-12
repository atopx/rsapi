use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;

use crate::jwt::Claims;
use crate::response::Empty;
use crate::response::{self};

#[tracing::instrument(skip(req, next))]
pub async fn authorization(mut req: Request, next: Next) -> Result<Response<Body>, Response<Body>> {
    // 修改错误类型
    // 从请求头中获取Authorization
    let auth_header = req.headers().get(header::AUTHORIZATION).and_then(|header| header.to_str().ok());

    // 如果没有找到有效的Authorization头，返回未授权错误
    let token = match auth_header {
        Some(token) => {
            let mut header = token.split_whitespace();
            match (header.next(), header.next()) {
                // 添加模式匹配防止unwrap panic
                (Some("Bearer"), Some(token)) => token,
                _ => {
                    return Err(response::Response::<Empty>::error(
                        "Unauthorized: Invalid token format",
                        StatusCode::UNAUTHORIZED,
                    )
                    .into_response()); // 转换为正确的响应类型
                }
            }
        }
        None => {
            return Err(response::Response::<Empty>::error(
                "Unauthorized: Missing token",
                StatusCode::UNAUTHORIZED,
            )
            .into_response()); // 转换为正确的响应类型
        }
    };
    // 验证token
    match Claims::verify(token) {
        Ok(claims) => {
            // 检查token是否已过期
            let now = chrono::Local::now().naive_local().and_utc().timestamp();
            if claims.exp < now {
                tracing::info!(username = claims.username, "verify token error: Token Expired");
                return Err(response::Response::<Empty>::error(
                    "Unauthorized: Token Expired",
                    StatusCode::UNAUTHORIZED,
                )
                .into_response());
            }
            tracing::info!(username = claims.username, "verify token success");

            // 将claims信息添加到请求扩展中，以便后续处理可以使用
            req.extensions_mut().insert(claims);
            // 继续处理请求
            Ok(next.run(req).await)
        }
        Err(e) => {
            tracing::error!(error=%e, "token error");
            Err(response::Response::<Empty>::error(
                "Unauthorized: Invalid token",
                StatusCode::UNAUTHORIZED,
            )
            .into_response()) // 转换为正确的响应类型
        }
    }
}
