pub mod appeal;
pub mod mutation;
pub mod visibility;

use axum::Router;
use sqlx::PgPool;

use crate::api::authority::{appeal::appeal, mutation::mutation, visibility::visibility};

pub fn authority() -> Router<PgPool> {
    Router::new()
        .nest("/appeal", appeal())
        .nest("/mutation", mutation())
        .nest("/visibility", visibility())
}