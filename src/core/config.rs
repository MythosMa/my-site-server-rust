use crate::error::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // 1. 获取本地开发环境标识
        let run_env = std::env::var("RUN_ENV").ok();

        // 2. 尝试加载文件
        if let Some(env) = run_env {
            // 本地开发模式：根据 RUN_ENV 加载 .env.development 等
            let env_file = format!(".env.{}", env);
            dotenvy::from_filename(&env_file).ok();
            println!("Local mode: Loading from {}", env_file);
        } else {
            // 生产环境模式（Docker）：直接尝试加载默认 .env（如果存在）
            // 如果是在 Docker 中运行，这里即便找不到文件也会因为 .ok() 忽略错误
            dotenvy::dotenv().ok();
        }

        // 3. 最终防线：从环境变量中提取
        // 原理：不管变量是来自 Docker 的 --env-file 还是来自 dotenvy
        // config::Environment::default() 都会统一从内存中的环境变量表里抓取数据
        let settings = config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(|e| anyhow::anyhow!("Config build failed: {}", e))?;

        settings
            .try_deserialize()
            .map_err(|e| anyhow::anyhow!("Config deserialize failed: {}", e).into())
    }
}
