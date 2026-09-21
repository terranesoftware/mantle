use sqlx::prelude::FromRow;

/// The database representation of the entries of a `BodyKind::Map`.
#[derive(FromRow)]
pub struct EntryRow {
    id: i64,
    map: i64,

    contains: i64,
    name: String,
    position: i64
}

impl EntryRow {
    keys!(map);

    /// Returns a copy of the contained `bodies` foreign key.
    pub fn contains(&self) -> i64 {
        self.contains
    }

    /// Returns a reference to the contained name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a copy of the contained position.
    pub fn position(&self) -> i64 {
        self.position
    }
}