pub mod dossier;
pub mod profile;
pub mod queries;

use sqlx::prelude::FromRow;
use time::OffsetDateTime;

/// The database representation of a `Varve`.
#[derive(FromRow)]
pub struct VarveRow {
    id: i64,
    account: i64,
    
    since: OffsetDateTime
}

impl VarveRow {
    keys!(account: i64);

    /// Returns a copy of the contained since datetime.
    pub fn since(&self) -> OffsetDateTime {
        self.since
    }
}