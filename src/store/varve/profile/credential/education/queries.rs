use framboid::account::profile::{credentials::status::Status as FramboidStatus, location::Location as FramboidLocation};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::{credential::education::{EducationRow, status::Status}, location::Location};

impl EducationRow {
    queries! {
        foreign = credential;

        param = education: (&str, &str, &FramboidLocation, &str, FramboidStatus);

        table = "educations";

        names = ["degree", "discipline", "location", "school", "status"];

        binds = [education.0, education.1, Location::from(education.2), education.3, Status::from(education.4)];
    }
}
