pub mod queries;

use sqlx::prelude::FromRow;

/// The database representation of `BodyKind::Integer`.
#[derive(FromRow)]
pub struct IntegerRow {
    id: i64,
    body: i64,

    integer: i64
}

impl IntegerRow {
    /// Returns a copy of the contained primary key.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns a copy of the contained foreign key.
    pub fn body(&self) -> i64 {
        self.body
    }

    /// Returns a copy of the contained integer.
    pub fn integer(&self) -> i64 {
        self.integer
    }
}