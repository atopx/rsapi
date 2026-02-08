use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Setting {
    pub service: ServiceSetting,
    pub logging: LoggingSetting,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            service: ServiceSetting {
                name: "api".to_string(),
                listen_addr: "localhost:3000".to_string(),
            },
            logging: LoggingSetting { level: "info".to_string(), format: "pretty".to_string() },
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceSetting {
    pub name: String,
    pub listen_addr: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoggingSetting {
    // RUST_LOG-style filter string, e.g. "info,sqlx=info,tower_http=trace"
    pub level: String,
    pub format: String, // pretty or json
}
