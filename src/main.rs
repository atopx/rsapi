use axum::Router;
use axum::routing;
use config::Config;
use rsapi::common::trace;
use rsapi::domain::example;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=info,tower_http=trace,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_file(true).with_line_number(true).with_target(true))
        .init();

    let config = Config::builder().add_source(config::File::with_name("config")).build().unwrap();
    let app = Router::new().route("/", routing::get(example::handler)).route_layer(axum::middleware::from_fn(trace::logging));
    
    let listener = tokio::net::TcpListener::bind(config.get_string("service.listen_addr").unwrap()).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
    println!("signal received, starting graceful shutdown");
}
