use crate::error::{AppError, Result};
use crate::models::User;
use axum::Json;

pub async fn create_user(Json(payload): Json<User>) -> Result<Json<User>> {
    if payload.username.is_empty() {
        return Err(AppError::Validation("Username cannot be empty".into()));
    }
    let user = User {
        id: Some(1),
        username: payload.username,
    };
    Ok(Json(user))
}
