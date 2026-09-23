pub mod queries;
pub mod status;

use sqlx::prelude::FromRow;

use crate::store::varve::profile::{credential::education::status::Status, location::Location};

#[derive(FromRow)]
pub struct EducationRow {
    id: i64,
    credential: i64,

    degree: String,
    discipline: String,
    location: Location,
    school: String,
    status: Status
}

impl EducationRow {
    keys!(credential);

    /// Returns a reference to the contained degree.
    pub fn degree(&self) -> &str {
        &self.degree
    }

    /// Returns a reference to the contained discipline.
    pub fn discipline(&self) -> &str {
        &self.discipline
    }

    /// Returns a reference to the contained location.
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Returns a reference to the contained school.
    pub fn school(&self) -> &str {
        &self.school
    }

    /// Returns a copy of the contained `Status`.
    pub fn status(&self) -> Status {
        self.status
    }
}
