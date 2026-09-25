pub mod queries;

use sqlx::prelude::FromRow;

/// The database representation of an `OrganizationKey`.
#[derive(FromRow)]
pub struct OrganizationRow {
    id: i64,

    organization: String
}

impl OrganizationRow {
    keys!();

    /// Returns a reference to the contained organization.
    pub fn organization(&self) -> &str {
        &self.organization
    }
}