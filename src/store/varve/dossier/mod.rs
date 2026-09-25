pub mod event;
pub mod queries;

use sqlx::prelude::FromRow;
use time::OffsetDateTime;

/// The database representation of a `Dossier`.
#[derive(FromRow)]
pub struct DossierRow {
    id: i64,
    varve: i64,

    from: OffsetDateTime,
    hash: Vec<u8>,
    position: i64,
    to: OffsetDateTime
}

impl DossierRow {
    keys!(varve: i64);

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