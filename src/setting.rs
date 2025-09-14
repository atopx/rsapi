use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Setting {
    pub service: ServiceSetting,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceSetting {
    pub listen_addr: String,
}
