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
    keys!(body: i64);

    /// Returns a copy of the contained integer.
    pub fn integer(&self) -> i64 {
        self.integer
    }
}