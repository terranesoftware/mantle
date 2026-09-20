pub mod queries;

use sqlx::prelude::FromRow;
use time::OffsetDateTime;

/// The database representation of a `Varve`.
#[derive(FromRow)]
pub struct VarveRow {
    id: i64,
    account: String,
    from: OffsetDateTime
}

impl VarveRow {
    /// Returns a copy of the contained id.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns a copy of the contained account key.
    pub fn account(&self) -> &str {
        &self.account
    }

    /// Returns a copy of the contained creation time.
    pub fn from(&self) -> OffsetDateTime {
        self.from
    }
}