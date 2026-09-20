pub mod queries;

use sqlx::prelude::FromRow;
use time::OffsetDateTime;

/// The database representation of an `Event`.
#[derive(FromRow)]
pub struct EventRow {
    id: i64,
    dossier: i64,

    from: OffsetDateTime,
    hash: Vec<u8>,
    position: i64,
    to: OffsetDateTime
}

impl EventRow {
    /// Returns a copy of the contained primary key.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns a copy of the contained foreign key.
    pub fn dossier(&self) -> i64 {
        self.dossier
    }

    /// Returns a copy of the contained from datetime.
    pub fn from(&self) -> OffsetDateTime {
        self.from
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
    pub fn to(&self) -> OffsetDateTime {
        self.to
    }
}