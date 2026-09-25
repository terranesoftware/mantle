pub mod queries;

use sqlx::prelude::FromRow;

/// The database representation of a `SourceKey`.
#[derive(FromRow)]
pub struct SourceRow {
    id: i64,

    source: String
}

impl SourceRow {
    keys!();

    /// Returns a reference to the contained source.
    pub fn source(&self) -> &str {
        &self.source
    }
}