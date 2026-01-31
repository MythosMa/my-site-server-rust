use crate::error::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // 1. 获取环境标识（仅用于本地开发时区分文件）
        // 如果环境变量中已经有了具体的配置（如 DATABASE_URL），
        // 那么 dotenvy 的加载失败也不会影响最终结果。
        if let Ok(run_env) = std::env::var("RUN_ENV") {
            let env_file = format!(".env.{}", run_env);
            dotenvy::from_filename(&env_file).ok();
            // .ok() 保证了如果文件不存在（比如在 Docker 里），程序不会崩溃
        } else {
            // 如果没设置 RUN_ENV，尝试加载默认的 .env
            dotenvy::dotenv().ok();
        }

        // 2. 核心：config 库的 Environment::default()
        // 原理：它会自动扫描当前进程中所有的环境变量。
        // 不管这些变量是来自 Docker 的 --env-file，还是来自 dotenvy 读取的文件，
        // 对程序来说它们现在都在操作系统的环境变量表里，待遇是一样的。
        let settings = config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build config: {}", e))?;

        settings
            .try_deserialize()
            .map_err(|e| anyhow::anyhow!("Failed to deserialize config: {}", e).into())
    }
}
