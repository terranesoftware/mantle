mod error;

use std::env::var;

use axum::serve;
use mantle::api::router;
use sqlx::PgPool;
use tokio::{main, net::TcpListener};

use crate::error::Error;

#[main]
async fn main() -> Result<(), Error> {
    let host = var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = var("PORT").unwrap_or("8000".to_string());
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

    let pool = PgPool::connect_lazy(&var("DATABASE").unwrap())?;

    let mantle = router().with_state(pool);

    serve(listener, mantle).await?;

    Ok(())
}