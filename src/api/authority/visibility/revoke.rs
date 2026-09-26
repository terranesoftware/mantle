use axum::Router;
use sqlx::PgPool;

pub fn revoke() -> Router<PgPool> {
    Router::new()
}