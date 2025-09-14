use axum::Router;
use axum::routing;
use config::Config as CfgLib;
use config::File as CfgFile;
use rsapi::app::AppStaate;
use rsapi::common::trace;
use rsapi::domain::example;
use rsapi::setting::Setting;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    let cfg = CfgLib::builder().add_source(CfgFile::with_name("setting")).build().unwrap();
    let setting: Setting = cfg.try_deserialize().unwrap();

    init_tracing(&setting);

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
    tracing::info!("signal received, starting graceful shutdown");
}

fn init_tracing(setting: &Setting) {
    // Prefer RUST_LOG if present; otherwise fall back to config level
    let env_filter = EnvFilter::try_from_env("RUST_LOG")
        .or_else(|_| EnvFilter::try_new(&setting.logging.level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    match setting.logging.format.as_str() {
        // structured JSON logs
        "json" => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    tracing_subscriber::fmt::layer()
                        .json()
                        .with_file(true)
                        .with_line_number(true)
                        .with_target(true),
                )
                .init();
        }
        // human-friendly pretty logs (default)
        _ => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    tracing_subscriber::fmt::layer()
                        .pretty()
                        .with_file(true)
                        .with_line_number(true)
                        .with_target(true),
                )
                .init();
        }
    };
}
