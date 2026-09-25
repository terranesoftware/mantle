pub mod kind;
pub mod prior;
pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::identity::name::usage::kind::Usage;

#[derive(FromRow)]
pub struct UsageRow {
    id: i64,
    name: i64,

    kind: Usage
}

impl UsageRow {
    keys!(name: i64);

    /// Returns a copy of the contained `Usage`.
    pub fn kind(&self) -> Usage {
        self.kind
    }
}
