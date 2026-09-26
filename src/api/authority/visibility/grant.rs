use axum::Router;
use sqlx::PgPool;

pub fn grant() -> Router<PgPool> {
    Router::new()
}