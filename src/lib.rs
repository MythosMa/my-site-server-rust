pub mod api;
pub mod core;
pub mod db;
pub mod error;
pub mod models;

use axum::Router;
use sqlx::MySqlPool;
use std::sync::Arc;

// 1. 定义应用全局共享状态
pub struct AppState {
    pub db: MySqlPool,
}

// 使用 Arc 包装状态，确保在多线程请求中高效共享
pub type SharedState = Arc<AppState>;

pub fn create_router(pool: MySqlPool) -> Router {
    let state = Arc::new(AppState { db: pool });

    Router::new()
        .nest("/", api::router(state.clone()))
        .with_state(state)
}
