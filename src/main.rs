use axum::Router;
use axum::routing;
use config::Config as CfgLib;
use config::File as CfgFile;
use rsapi::app::AppStaate;
use rsapi::common::trace;
use rsapi::domain::example;
use rsapi::setting::Setting;
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

    let cfg = CfgLib::builder().add_source(CfgFile::with_name("setting")).build().unwrap();
    let setting: Setting = cfg.try_deserialize().unwrap();

    let listener = tokio::net::TcpListener::bind(&setting.service.listen_addr).await.unwrap();
    tracing::info!("listening on {}", &setting.service.listen_addr);

    let app = Router::new()
        .route("/", routing::get(example::handler))
        .route_layer(axum::middleware::from_fn(trace::logging))
        .with_state(AppStaate::new(setting));

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
    println!("signal received, starting graceful shutdown");
}
