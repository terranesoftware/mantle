pub mod me;
pub mod new;

use axum::{Router, routing::{get, post}};
use sqlx::PgPool;

use crate::api::namespace::key::account::{me::me, new::new};

pub fn account() -> Router<PgPool> {
    Router::new()
        .route("/", get(me))
        .route("/{name}", post(new))
}