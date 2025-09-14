use std::sync::Arc;

use crate::setting::Setting;

#[derive(Debug, Clone)]
pub struct AppStaate {
    pub setting: Arc<Setting>,
}

impl AppStaate {
    pub fn new(setting: Setting) -> Self { AppStaate { setting: Arc::new(setting) } }
}
