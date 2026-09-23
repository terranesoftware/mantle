use crate::store::varve::profile::compensation::CompensationRow;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

impl CompensationRow {
    queries! {
        foreign = profile;

        table = "compensations";

        names = [];

        binds = [];

        recurse = {};
    }
}