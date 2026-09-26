use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}};
use framboid::account::Varve;
use sqlx::PgPool;

use crate::{api::{error::MantleError, utils::Bitcode}, store::varve::VarveRow};

pub async fn new(State(pool): State<PgPool>, Bitcode(body): Bitcode<Varve>) -> Result<Response, MantleError> {
    VarveRow::insert(&pool, &body).await?;

    Ok(
        (StatusCode::OK, "Varve created successfully").into_response()
    )
}