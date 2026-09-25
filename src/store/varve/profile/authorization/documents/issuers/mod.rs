pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::location::Location;

#[derive(FromRow)]
pub struct IssuerRow {
    id: i64,
    document: i64,

    name: String,
    jurisdiction: Location
}

impl IssuerRow {
    keys!(document: i64);

    /// Returns a reference to the contained name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the contained jurisdiction.
    pub fn jurisdiction(&self) -> &Location {
        &self.jurisdiction
    }
}