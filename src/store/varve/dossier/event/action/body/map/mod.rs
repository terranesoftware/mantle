pub mod queries;

use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct MapRow {
    id: i64,
    body: i64
}

impl MapRow {
    keys!(body);
}