pub mod grant;
pub mod revoke;

use axum::Router;
use sqlx::PgPool;

use crate::api::authority::visibility::{grant::grant, revoke::revoke};

pub fn visibility() -> Router<PgPool> {
    Router::new()
        .nest("/grant", grant())
        .nest("/revoke", revoke())
}