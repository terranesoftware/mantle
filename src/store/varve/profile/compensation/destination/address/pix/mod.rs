pub mod queries;

use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct PixRow {
    id: i64,
    address: i64,

    identifier: String
}

impl PixRow {
    keys!(address: i64);

    /// Returns a reference to the contained identifier.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}
