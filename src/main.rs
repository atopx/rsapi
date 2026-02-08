use axum::Router;
use axum::routing;
use config::Config as CfgLib;
use config::File as CfgFile;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer as _;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod app;
pub mod common;
pub mod domain;
pub mod setting;

#[tokio::main]
async fn main() {
    let cfg = CfgLib::builder()
        .add_source(CfgFile::with_name("setting").required(false))
        .add_source(CfgFile::with_name("config").required(false))
        .build()
        .unwrap_or_default();
    let setting: setting::Setting = cfg.try_deserialize().unwrap_or_default();

    init_tracing(&setting);

    let listener = tokio::net::TcpListener::bind(&setting.service.listen_addr).await.unwrap();
    tracing::info!("listening on {}", &setting.service.listen_addr);

    let app = Router::new()
        .route("/", routing::get(domain::example::handler))
        .route_layer(axum::middleware::from_fn(common::trace::logging))
        .with_state(app::AppStaate::new(setting));

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
    tracing::info!("signal received, starting graceful shutdown");
}

fn init_tracing(setting: &setting::Setting) {
    // Prefer RUST_LOG if present; otherwise fall back to config level
    let env_filter = EnvFilter::try_from_env("RUST_LOG")
        .or_else(|_| EnvFilter::try_new(&setting.logging.level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // W3C trace context propagation for cross-service trace continuity.
    global::set_text_map_propagator(TraceContextPropagator::new());

    let resource = Resource::builder().with_service_name(setting.service.name.clone()).build();
    let tracer_provider = SdkTracerProvider::builder().with_resource(resource).build();
    let tracer = tracer_provider.tracer(setting.service.name.clone());

    match setting.logging.format.as_str() {
        // structured JSON logs
        "json" => {
            tracing_subscriber::registry()
                .with(tracing_opentelemetry::layer().with_tracer(tracer.clone()))
                .with(
                    tracing_subscriber::fmt::layer()
                        .json()
                        .flatten_event(true)
                        .with_current_span(true)
                        .with_span_list(true)
                        .with_file(true)
                        .with_line_number(true)
                        .with_target(true)
                        .with_filter(env_filter),
                )
                .init();
        }
        // human-friendly pretty logs (default)
        _ => {
            tracing_subscriber::registry()
                .with(tracing_opentelemetry::layer().with_tracer(tracer))
                .with(
                    tracing_subscriber::fmt::layer()
                        .pretty()
                        .with_file(true)
                        .with_line_number(true)
                        .with_target(true)
                        .with_filter(env_filter),
                )
                .init();
        }
    };
}
