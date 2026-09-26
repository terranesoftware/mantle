pub mod file;
pub mod status;

use axum::{Router, routing::{get, post}};
use sqlx::PgPool;

use crate::api::authority::appeal::{file::file, status::status};

pub fn appeal() -> Router<PgPool> {
    Router::new()
        .route("/", get(status))
        .route("/", post(file))
}