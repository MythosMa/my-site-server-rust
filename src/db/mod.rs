use crate::error::Result;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn init_pool(database_url: &str) -> Result<MySqlPool> {
    // 原理：sqlx::mysql::MySqlPool 内部实现了 Arc 引用计数
    // 它代表一个异步数据库连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to MySQL: {}", e))?;

    Ok(pool)
}
