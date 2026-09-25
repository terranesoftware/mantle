use std::{env::var, io::Result};

use axum::{Router, serve};
use tokio::{main, net::TcpListener};

#[main]
async fn main() -> Result<()> {
    let host = var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = var("PORT").unwrap_or("8000".to_string());
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

    let network = Router::new();

    serve(listener, network).await
}