pub mod file;
pub mod status;

use axum::{Router, routing::{get, post}};

use crate::api::authority::appeal::{file::file, status::status};

pub fn appeal() -> Router {
    Router::new()
        .route("/", get(status))
        .route("/", post(file))
}