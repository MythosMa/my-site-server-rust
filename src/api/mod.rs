pub mod sub_web;
pub mod user;
pub mod work;

use crate::SharedState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router(_state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", post(user::create_user))
        .route("/sub-webs", get(sub_web::list_sub_web))
        .route("/word-clouds", get(work::list_word_cloud))
        .route("/works", get(work::list_work))
}
