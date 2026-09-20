pub mod queries;

use sqlx::prelude::FromRow;
use time::OffsetDateTime;

/// The database representation of an `Action`.
#[derive(FromRow)]
pub struct ActionRow {
    id: i64,
    event: i64,

    hash: Vec<u8>,
    name: String,
    position: i64,
    source: String,
    time: OffsetDateTime
}

impl ActionRow {
    /// Returns a copy of the contained primary key.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns a copy of the contained foreign key.
    pub fn event(&self) -> i64 {
        self.event
    }

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

    /// Returns a reference to the contained source.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns a copy of the contained datetime.
    pub fn time(&self) -> OffsetDateTime {
        self.time
    }
}