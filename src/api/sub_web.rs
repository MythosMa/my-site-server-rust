use crate::SharedState;
use crate::error::Result;
use crate::models::{ApiResponse, SubWeb};
use axum::{Json, extract::State};
use tracing::{error, info};

pub async fn list_sub_webs(
    State(state): State<SharedState>,
) -> Result<Json<ApiResponse<Vec<SubWeb>>>> {
    info!("Fetching all sub_web records from database..."); // 记录开始

    // 1. 执行 SQL 查询
    // 原理：sqlx::query_as 会自动将结果集的每一行映射为 SubWeb 结构体
    // 这里我们使用 &state.db 拿到连接池的引用
    let sub_webs = sqlx::query_as::<_, SubWeb>("SELECT id, name, url FROM sub_web")
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            error!("Failed to query sub_web: {}", e); // 记录错误
            anyhow::anyhow!(e)
        })?;

    info!("Successfully retrieved {} records", sub_webs.len()); // 记录结果

    // 2. 包装为统一的返回格式
    let response = ApiResponse::success(sub_webs);

    // 3. 返回 JSON
    Ok(Json(response))
}
