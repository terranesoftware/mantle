use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}};
use framboid::keys::{Key, account::AccountKey};
use sqlx::PgPool;

use crate::{api::{error::MantleError, utilities::{authentication::anonymous::Anonymous, bitcode::Bitcode}}, store::account::AccountRow};

// This needs account creation through usernames/passwords or whatever
pub async fn new(
    _: Anonymous,
    State(pool): State<PgPool>,
    Bitcode(account): Bitcode<AccountKey>
) -> Result<Response, MantleError> {
    AccountRow::insert(&pool, None, &account).await?;

    Ok(
        (StatusCode::OK, format!("Key {} claimed", account.key())).into_response()
    )
}