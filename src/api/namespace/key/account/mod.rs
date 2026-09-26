use axum::{Router, extract::Path, routing::{get, post}};
use sqlx::PgPool;

pub fn account() -> Router<PgPool> {
    Router::new()
        .route("/", get(me))
        .route("/{name}", post(new))
}

pub async fn me() {
    
}

pub async fn new(Path(name): Path<String>) {
    
}