use axum::extract::Path;

pub async fn fetch(Path(name): Path<String>) {
    
}