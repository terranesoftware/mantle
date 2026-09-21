pub mod queries;

use sqlx::prelude::FromRow;

/// The database representation of `BodyKind::Boolean`.
#[derive(FromRow)]
pub struct BooleanRow {
    id: i64,
    body: i64,

    boolean: bool
}

impl BooleanRow {
    /// Returns a copy of the contained primary key.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns a copy of the contained foreign key.
    pub fn body(&self) -> i64 {
        self.body
    }

    /// Returns a copy of the contained boolean.
    pub fn boolean(&self) -> bool {
        self.boolean
    }
}