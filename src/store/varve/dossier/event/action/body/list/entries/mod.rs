pub mod queries;

use sqlx::prelude::FromRow;

/// The database representation of the entries of a `BodyKind::List`.
#[derive(FromRow)]
pub struct EntryRow {
    id: i64,
    list: i64,

    contains: i64,
    position: i64
}

impl EntryRow {
    keys!(list);

    /// Returns a copy of the contained `bodies` foreign key.
    pub fn contains(&self) -> i64 {
        self.contains
    }

    /// Returns a copy of the contained position.
    pub fn position(&self) -> i64 {
        self.position
    }
}