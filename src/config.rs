use std::sync::OnceLock;

#[derive(Debug)]
pub struct AppConfig {
    pub server: Server,
    pub crontab: Crontab,
}

#[derive(Debug)]
pub struct Server {
    pub version: String,
    pub listen_addr: String,
    pub database_url: String,
}

#[derive(Debug)]
pub struct Crontab {
    pub example: String,
}

static CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// 获取全局配置（惰性初始化）
pub fn get() -> &'static AppConfig {
    CONFIG.get_or_init(|| AppConfig {
        server: Server {
            version: std::env::var("VERSION").unwrap_or_default(),
            listen_addr: std::env::var("LISTEN_ADDR").unwrap_or_default(),
            database_url: std::env::var("DATABASE_URL").unwrap_or_default(),
        },
        crontab: Crontab { example: std::env::var("CRONTAB_EXAMPLE").unwrap() },
    })
}
