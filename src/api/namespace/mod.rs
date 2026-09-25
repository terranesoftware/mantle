pub mod affiliation;
pub mod key;

use axum::Router;

use crate::api::namespace::{affiliation::affiliation, key::key};

pub fn namespace() -> Router {
    Router::new()
        .nest("/affiliation", affiliation())
        .nest("/key", key())
}