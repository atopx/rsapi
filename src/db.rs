use std::sync::OnceLock;

use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::entity::prelude::*;
use tracing::info;

// 全局数据库连接句柄
pub static DB_CONN: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init(dsn: &str) -> Result<(), DbErr> {
    let mut opt = ConnectOptions::new(dsn);
    opt.connect_timeout(std::time::Duration::from_secs(10))
        .max_lifetime(std::time::Duration::from_secs(60 * 60))
        .max_connections(20)
        .min_connections(1)
        .idle_timeout(std::time::Duration::from_secs(60 * 5))
        .acquire_timeout(std::time::Duration::from_secs(5));
    let conn = Database::connect(dsn).await?;
    DB_CONN.set(conn).expect("Failed to set DB connection");
    info!("Database connection initialized");
    Ok(())
}

pub fn get() -> &'static DatabaseConnection { DB_CONN.get().expect("Database not initialized") }
