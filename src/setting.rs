use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Setting {
    pub service: ServiceSetting,
    pub logging: LoggingSetting,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceSetting {
    pub listen_addr: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoggingSetting {
    // RUST_LOG-style filter string, e.g. "info,sqlx=info,tower_http=trace"
    pub level: String,
    pub format: String,
}
