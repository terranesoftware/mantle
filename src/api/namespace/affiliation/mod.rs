pub mod end;
pub mod resolve;
pub mod start;

use axum::{Router, routing::{get, post}};

use crate::api::namespace::affiliation::{end::end, resolve::resolve, start::start};

pub fn affiliation() -> Router {
    Router::new()
        .route("/end", post(end))
        .route("/resolve", get(resolve))
        .route("/start", post(start))
}