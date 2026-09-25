pub mod arrangement;
pub mod employment;
pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::{
    engagement::{arrangement::Arrangement, employment::Employment},
    location::Location,
    period::Period
};

#[derive(FromRow)]
pub struct EngagementRow {
    id: i64,
    profile: i64,

    arrangement: Arrangement,
    employer: String,
    employment: Employment,
    location: Option<Location>,
    period: Period,
    title: String
}

impl EngagementRow {
    keys!(profile: i64);

    /// Returns a copy of the contained `Arrangement`.
    pub fn arrangement(&self) -> Arrangement {
        self.arrangement
    }

    /// Returns a reference to the contained employer.
    pub fn employer(&self) -> &str {
        &self.employer
    }

    /// Returns a copy of the contained `Employment`.
    pub fn employment(&self) -> Employment {
        self.employment
    }

    /// Returns a reference to the contained location if present.
    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }

    /// Returns a reference to the contained period.
    pub fn period(&self) -> &Period {
        &self.period
    }

    /// Returns a reference to the contained title.
    pub fn title(&self) -> &str {
        &self.title
    }
}
