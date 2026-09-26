pub mod fetch;
pub mod new;

use axum::{Router, routing::{get, post}};
use sqlx::PgPool;

use crate::api::varve::{fetch::fetch, new::new};

pub fn varve() -> Router<PgPool> {
    Router::new()
        .route("/{name}", get(fetch))
        .route("/new", post(new))
}