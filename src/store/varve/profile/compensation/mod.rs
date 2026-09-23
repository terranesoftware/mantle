pub mod destination;
pub mod queries;

use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct CompensationRow {
    id: i64,
    profile: i64
}

impl CompensationRow {
    keys!(profile);
}