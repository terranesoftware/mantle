pub mod authentication;
pub mod authority;
pub mod error;
pub mod namespace;
pub mod utilities;
pub mod varve;

use axum::{Router, middleware::from_fn};
use sqlx::PgPool;

use crate::api::{authentication::authentication, authority::authority, namespace::namespace, utilities::principal::principal, varve::varve};

pub fn router() -> Router<PgPool> {
    Router::new()
        .nest("/authentication", authentication())
        .nest("/authority", authority())
        .nest("/namespace", namespace())
        .nest("/varve", varve())
        .layer(from_fn(principal))
}