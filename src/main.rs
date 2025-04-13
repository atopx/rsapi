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

    schedule::start(&config.crontab)
        .await
        .expect("scheduler start failed");

    let app = router::new();

    let listener = tokio::net::TcpListener::bind(&config.server.listen_addr)
        .await
        .unwrap();
    tracing::info!(
        "listening on {}, api version {}",
        listener.local_addr().unwrap(),
        &config.server.version
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    // tokio 提供的 ctrl-c 信号监听
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
    println!("signal received, starting graceful shutdown");
}
