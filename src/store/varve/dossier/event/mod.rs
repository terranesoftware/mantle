pub mod action;
pub mod queries;

use sqlx::prelude::FromRow;
use time::OffsetDateTime;

/// The database representation of an `Event`.
#[derive(FromRow)]
pub struct EventRow {
    id: i64,
    dossier: i64,

    since: OffsetDateTime,
    hash: Vec<u8>,
    position: i64,
    until: OffsetDateTime
}

impl EventRow {
    keys!(dossier: i64);

    /// Returns a copy of the contained since datetime.
    pub fn since(&self) -> OffsetDateTime {
        self.since
    }

    /// Returns a reference to the contained hash.
    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    /// Returns a copy of the contained position.
    pub fn position(&self) -> i64 {
        self.position
    }

    /// Returns a copy of the contained to datetime.
    pub fn until(&self) -> OffsetDateTime {
        self.until
    }
}