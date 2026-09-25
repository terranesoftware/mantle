pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::location::Location;

#[derive(FromRow)]
pub struct AddressRow {
    id: i64,
    identity: i64,

    lines: Vec<String>,
    location: Location,
    postcode: String
}

impl AddressRow {
    keys!(identity: i64);

    /// Returns a reference to the contained address lines.
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Returns a reference to the contained location.
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Returns a reference to the contained postcode.
    pub fn postcode(&self) -> &str {
        &self.postcode
    }
}
