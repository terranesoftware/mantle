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
    keys!(body);

    /// Returns a copy of the contained boolean.
    pub fn boolean(&self) -> bool {
        self.boolean
    }
}