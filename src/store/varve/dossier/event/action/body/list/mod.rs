pub mod entries;
pub mod queries;

use sqlx::prelude::FromRow;

/// The database representation of `BodyKind::List`.
#[derive(FromRow)]
pub struct ListRow {
    id: i64,
    body: i64
}

impl ListRow {
    keys!(body);
}