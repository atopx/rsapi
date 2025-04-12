use std::time::Duration;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::Response;
use axum::middleware;
use axum::routing::get;
use axum::routing::post;
use tower_http::classify::ServerErrorsFailureClass;

use crate::middle;
use crate::service;

pub fn auth_routes() -> Router {
    Router::new().route("/claims", get(service::claims)).layer(middleware::from_fn(middle::authorization))
}

pub fn no_auth_routes() -> Router { Router::new().route("/login", post(service::login)) }

pub fn new() -> Router {
    let trace_layer = tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(
            |_: &Request<Body>| tracing::info_span!("API", trace_id = %uuid::Uuid::new_v4().to_string()),
        )
        .on_request(|req: &Request<Body>, span: &tracing::Span| {
            tracing::debug!(parent: span, method = %req.method(), uri = %req.uri(), "request");
        })
        .on_response(|resp: &Response<Body>, latency: Duration, span: &tracing::Span| {
            tracing::debug!( parent: span, latency = ?latency, status = %resp.status(), "response");
        })
        .on_failure(|error: ServerErrorsFailureClass, latency: Duration, span: &tracing::Span| {
            tracing::error!(parent: span, latency = ?latency, error = %error, "handler failure");
        });

    Router::new()
        .nest("/api", auth_routes())
        .nest("/api", no_auth_routes())
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(trace_layer)
        .layer(middleware::from_fn(middle::error_handler))
}
