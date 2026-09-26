mod error;

use std::env::var;

use axum::serve;
use mantle::api::router;
use sqlx::PgPool;
use tokio::{main, net::TcpListener};

use crate::error::Error;

#[main]
async fn main() -> Result<(), Error> {
    let host = var("HOST").unwrap();
    let port = var("PORT").unwrap();
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

    let pool = PgPool::connect_lazy(&var("DATABASE").unwrap())?;

    let mantle = router().with_state(pool);

    serve(listener, mantle).await?;

    Ok(())
}