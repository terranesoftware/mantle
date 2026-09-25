use axum::extract::Path;

pub async fn claim(Path(name): Path<String>) {
    
}