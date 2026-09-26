pub mod affiliation;
pub mod key;

use axum::Router;
use sqlx::PgPool;

use crate::api::namespace::{affiliation::affiliation, key::key};

pub fn namespace() -> Router<PgPool> {
    Router::new()
        .nest("/affiliation", affiliation())
        .nest("/key", key())
}