pub mod queries;

use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct AuthorizationRow {
    id: i64,
    profile: i64
}

impl AuthorizationRow {
    keys!(profile);
}