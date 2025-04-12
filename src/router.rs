use axum::Router;
use axum::middleware;
use axum::routing::get;
use axum::routing::post;

use crate::middle::authorization;
use crate::service;

pub fn auth_routes() -> Router {
    Router::new().route("/claims", get(service::claims)).layer(middleware::from_fn(authorization))
}

pub fn no_auth_routes() -> Router { Router::new().route("/login", post(service::login)) }
