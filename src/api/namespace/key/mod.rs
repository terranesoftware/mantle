pub mod account;
pub mod organization;
pub mod resolve;
pub mod source;

use axum::{Router, routing::get};

use crate::api::namespace::{affiliation::resolve::resolve, key::{account::account, organization::organization, source::{source, sources}}};

pub fn key() -> Router {
    Router::new()
        .nest("/account", account())
        .nest("/organization", organization())
        .route("resolve/{key}", get(resolve))
        .nest("/source", source())
}