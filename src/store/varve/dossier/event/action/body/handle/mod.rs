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
    /// Returns a copy of the contained primary key.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns a copy of the contained foreign key.
    pub fn body(&self) -> i64 {
        self.body
    }

    /// Returns a reference to the contained handle.
    pub fn handle(&self) -> &str {
        &self.handle
    }
}