pub mod authority;
pub mod namespace;
pub mod varve;

use axum::Router;

use crate::api::{authority::authority, namespace::namespace, varve::varve};

pub fn router() -> Router {
    Router::new()
        .nest("/authority", authority())
        .nest("/namespace", namespace())
        .nest("/varve", varve())
}