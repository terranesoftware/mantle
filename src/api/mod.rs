pub mod authentication;
pub mod authority;
pub mod namespace;
pub mod varve;

use axum::Router;

use crate::api::{authentication::authentication, authority::authority, namespace::namespace, varve::varve};

pub fn router() -> Router {
    Router::new()
        .nest("/authentication", authentication())
        .nest("/authority", authority())
        .nest("/namespace", namespace())
        .nest("/varve", varve())
}