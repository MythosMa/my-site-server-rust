use my_site_server_rust::{core::Config, core::logging, create_router, db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载环境变量与配置
    let config = Config::from_env()?;

    logging::init();

    // 2. 初始化数据库连接池
    let pool = db::init_pool(&config.database_url).await?;
    println!("Successfully connected to MySQL!");

    // 3. 启动路由（注入连接池）
    let app = create_router(pool);

    // 4. 绑定端口并运行
    let addr = format!("127.0.0.1:{}", config.server_port);
    let listener = TcpListener::bind(&addr).await?;
    println!("listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
