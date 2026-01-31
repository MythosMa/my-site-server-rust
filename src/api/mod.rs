pub mod sub_web;
pub mod user;

use crate::SharedState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router(_state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", post(user::create_user))
        .route("/sub-web", get(sub_web::list_sub_webs))
}
