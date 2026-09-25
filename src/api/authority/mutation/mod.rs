pub mod grant;
pub mod relinquish;

use axum::{Router, routing::post};

use crate::api::authority::mutation::{grant::grant, relinquish::relinquish};

pub fn mutation() -> Router {
    Router::new()
        .route("/grant", post(grant))
        .route("/relinquish", post(relinquish))
}