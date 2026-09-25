use axum::{Router, extract::Path, routing::{get, post}};

pub fn account() -> Router {
    Router::new()
        .route("/", get(me))
        .route("/{name}", post(new))
}

pub async fn me() {
    
}

pub async fn new(Path(name): Path<String>) {
    
}