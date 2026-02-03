use anyhow::anyhow;
use axum::{Json, extract::State};
use tracing::{error, info};

use crate::{
    SharedState,
    error::Result,
    models::{ApiResponse, WordCloud, Work},
};

pub async fn list_word_cloud(
    State(state): State<SharedState>,
) -> Result<Json<ApiResponse<Vec<WordCloud>>>> {
    info!("开始查询词云数据");
    let word_cloud = sqlx::query_as::<_, WordCloud>("SELECT id, text, value from word_cloud")
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            error!("查询词云数据失败: {}", e);
            anyhow!(e)
        })?;

    info!("查询到{}条词云数据", word_cloud.len());

    let response = ApiResponse::success(word_cloud);

    Ok(Json(response))
}

pub async fn list_work(State(state): State<SharedState>) -> Result<Json<ApiResponse<Vec<Work>>>> {
    info!("开始查询作品数据");
    let work = sqlx::query_as::<_, Work>("SELECT id, year, description from work")
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            error!("查询作品数据失败: {}", e);
            anyhow!(e)
        })?;

    info!("查询到{}条作品数据", work.len());

    let response = ApiResponse::success(work);

    Ok(Json(response))
}
