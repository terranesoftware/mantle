pub mod grant;
pub mod relinquish;

use axum::{Router, routing::post};
use sqlx::PgPool;

use crate::api::authority::mutation::{grant::grant, relinquish::relinquish};

pub fn mutation() -> Router<PgPool> {
    Router::new()
        .route("/grant", post(grant))
        .route("/relinquish", post(relinquish))
}