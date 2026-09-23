use framboid::account::profile::compensation::registration::Registration;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::{compensation::registration::RegistrationRow, location::Location};

impl RegistrationRow {
    queries! {
        foreign = compensation;

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
