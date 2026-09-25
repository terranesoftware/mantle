use framboid::account::profile::compensation::registration::Registration;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::profile::{compensation::registration::RegistrationRow, location::Location};

impl RegistrationRow {
    queries! {
        foreign = compensation: i64;

        param = registration: &Registration;

        table = "registrations";

        names = ["scheme", "identifier", "jurisdiction"];

        binds = [
            registration.scheme(),
            registration.identifier(),
            Location::from(registration.jurisdiction())
        ];
    }
}
