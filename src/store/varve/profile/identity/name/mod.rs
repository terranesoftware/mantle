pub mod identifiers;
pub mod queries;
pub mod usage;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::identity::name::identifiers::Identifiers;

#[derive(FromRow)]
pub struct NameRow {
    id: i64,
    identity: i64,

    latin: Option<Identifiers>,
    native: Identifiers
}

impl NameRow {
    keys!(identity: i64);

    /// Returns a reference to the contained latin identifiers if present.
    pub fn latin(&self) -> Option<&Identifiers> {
        self.latin.as_ref()
    }

    /// Returns a reference to the contained native identifiers.
    pub fn native(&self) -> &Identifiers {
        &self.native
    }
}
