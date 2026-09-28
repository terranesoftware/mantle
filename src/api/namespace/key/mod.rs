pub mod account;
pub mod organization;
pub mod resolve;
pub mod source;

use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::api::namespace::key::{account::{me::me, new::new}, organization::organization, resolve::resolve, source::source};

pub fn key() -> Router<PgPool> {
    Router::new()
        .route("/account", get(me).post(new))
        .nest("/organization", organization())
        .route("resolve/{key}", get(resolve))
        .nest("/source", source())
}