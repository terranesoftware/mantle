use axum::extract::Path;

pub async fn new(Path(name): Path<String>) {
    
}