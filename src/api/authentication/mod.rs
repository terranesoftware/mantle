pub mod bind;
pub mod unbind;

use axum::{Router, routing::post};

use crate::api::authentication::{bind::bind, unbind::unbind};

pub fn authentication() -> Router {
    Router::new()
        .route("/bind", post(bind))
        .route("/unbind", post(unbind))
}