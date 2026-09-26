pub mod authentication;
pub mod authority;
pub mod error;
pub mod namespace;
pub mod utils;
pub mod varve;

use axum::Router;
use sqlx::PgPool;

use crate::api::{authentication::authentication, authority::authority, namespace::namespace, varve::varve};

pub fn router() -> Router<PgPool> {
    Router::new()
        .nest("/authentication", authentication())
        .nest("/authority", authority())
        .nest("/namespace", namespace())
        .nest("/varve", varve())
}