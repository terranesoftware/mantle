pub mod queries;

use sqlx::prelude::FromRow;

/// The database representation of `BodyKind::Handle`.
#[derive(FromRow)]
pub struct HandleRow {
    id: i64,
    body: i64,

    handle: String
}

impl HandleRow {
    keys!(body: i64);

    /// Returns a reference to the contained handle.
    pub fn handle(&self) -> &str {
        &self.handle
    }
}