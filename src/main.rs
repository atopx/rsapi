use std::time::Duration;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::Response;
use axum::middleware;
use tower_http::classify::ServerErrorsFailureClass;
use tracing::Span;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::OffsetTime;

mod config;
mod db;
mod jwt;
mod middle;
mod model;
mod response;
mod router;
mod schedule;
mod service;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug,sqlx=warn"));

    tracing_subscriber::fmt()
        .with_timer(OffsetTime::local_rfc_3339().unwrap())
        .with_env_filter(filter)
        .init();

    let config = config::get();

    if let Err(e) = db::init(&config.server.database_url).await {
        panic!("Cloud not init database: {e}");
    }

    schedule::start(&config.crontab).await.expect("scheduler start failed");

    let trace_layer = tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(
            |_: &Request<Body>| tracing::info_span!("API", trace_id = %uuid::Uuid::new_v4().to_string()),
        )
        .on_request(|req: &Request<Body>, span: &Span| {
            tracing::debug!(parent: span, method = %req.method(), uri = %req.uri(), "request");
        })
        .on_response(|resp: &Response<Body>, latency: Duration, span: &Span| {
            tracing::debug!( parent: span, latency = ?latency, status = %resp.status(), "response");
        })
        .on_failure(|error: ServerErrorsFailureClass, latency: Duration, span: &Span| {
            tracing::error!(parent: span, latency = ?latency, error = %error, "handler failure");
        });

    let app = Router::new()
        .nest("/api", router::auth_routes())
        .nest("/api", router::no_auth_routes())
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(trace_layer)
        .layer(middleware::from_fn(middle::error_handler));

    let listener = tokio::net::TcpListener::bind(&config.server.listen_addr).await.unwrap();
    tracing::info!(
        "listening on {}, api version {}",
        listener.local_addr().unwrap(),
        &config.server.version
    );
    axum::serve(listener, app).await.unwrap();
}
