use sqlx::prelude::FromRow;

use crate::store::varve::profile::compensation::destination::kind::Destination;

pub mod kind;
pub mod queries;

#[derive(FromRow)]
pub struct DestinationRow {
    id: i64,
    compensation: i64,

    kind: Destination
}

impl DestinationRow {
    keys!(compensation);

    /// Returns a copy of the contained `Destination`.
    pub fn kind(&self) -> Destination {
        self.kind
    }
}