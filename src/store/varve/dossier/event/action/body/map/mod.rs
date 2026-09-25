pub mod entries;
pub mod queries;

use sqlx::prelude::FromRow;

/// The database representation of `BodyKind::Map`.
#[derive(FromRow)]
pub struct MapRow {
    id: i64,
    body: i64
}

impl MapRow {
    keys!(body: i64);
}