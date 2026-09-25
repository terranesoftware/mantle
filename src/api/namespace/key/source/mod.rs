pub mod claim;
pub mod owner;
pub mod sources;

use axum::{Router, routing::{get, post}};

use crate::api::namespace::key::source::{claim::claim, owner::owner, sources::sources};

pub fn source() -> Router {
    Router::new()
        .route("/claim/{name}", post(claim))
        .route("/{organization}", get(owner))
        .route("/", get(sources))
}