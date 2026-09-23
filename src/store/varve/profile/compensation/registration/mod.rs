pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::location::Location;

#[derive(FromRow)]
pub struct RegistrationRow {
    id: i64,
    compensation: i64,

    scheme: String,
    identifier: String,
    jurisdiction: Location
}

impl RegistrationRow {
    keys!(compensation);

    /// Returns a reference to the contained scheme.
    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    /// Returns a reference to the contained identifier.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns a reference to the contained jurisdiction.
    pub fn jurisdiction(&self) -> &Location {
        &self.jurisdiction
    }
}
