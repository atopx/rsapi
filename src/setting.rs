use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Setting {
    pub service: ServiceSetting,
    pub logging: LoggingSetting,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceSetting {
    #[serde(default = "default_service_name")]
    pub name: String,
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoggingSetting {
    // RUST_LOG-style filter string, e.g. "info,sqlx=info,tower_http=trace"
    #[serde(default = "default_logging_level")]
    pub level: String,
    #[serde(default = "default_logging_format")]
    pub format: String, // pretty or json
}

fn default_service_name() -> String { "api".to_string() }

fn default_listen_addr() -> String { "localhost:3000".to_string() }

fn default_logging_level() -> String { "info".to_string() }

fn default_logging_format() -> String { "pretty".to_string() }
