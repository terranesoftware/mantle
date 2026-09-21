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
    /// Returns a copy of the contained primary key.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns a copy of the contained foreign key.
    pub fn body(&self) -> i64 {
        self.body
    }
}