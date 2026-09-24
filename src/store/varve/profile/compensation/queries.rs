use crate::store::varve::profile::compensation::{CompensationRow, destination::DestinationRow, registration::RegistrationRow};
use framboid::account::profile::compensation::Compensation;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

impl CompensationRow {
    queries! {
        foreign = profile;

        param = compensation: &Compensation;

        table = "compensations";

        names = [];

        binds = [];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            for destination in compensation.destinations() {
                DestinationRow::insert(pool, row.id, destination).await?;
            }

            for registration in compensation.registrations() {
                RegistrationRow::insert(pool, row.id, registration).await?;
            }

            Ok(())
        };
    }
}