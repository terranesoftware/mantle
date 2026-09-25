pub mod body;
pub mod queries;

use sqlx::prelude::FromRow;
use time::OffsetDateTime;

/// The database representation of an `Action`.
#[derive(FromRow)]
pub struct ActionRow {
    id: i64,
    event: i64,
    source: i64,

    hash: Vec<u8>,
    name: String,
    position: i64,
    time: OffsetDateTime
}

impl ActionRow {
    keys!(event: i64, source: i64);

    /// Returns a reference to the contained hash.
    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    /// Returns a reference to the contained name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a copy of the contained position.
    pub fn position(&self) -> i64 {
        self.position
    }

    /// Returns a copy of the contained datetime.
    pub fn time(&self) -> OffsetDateTime {
        self.time
    }
}