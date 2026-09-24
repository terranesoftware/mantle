pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::period::Period;

#[derive(FromRow)]
pub struct PriorRow {
    id: i64,
    usage: i64,

    period: Period
}

impl PriorRow {
    keys!(usage);

    /// Returns a reference to the contained period.
    pub fn period(&self) -> &Period {
        &self.period
    }
}
