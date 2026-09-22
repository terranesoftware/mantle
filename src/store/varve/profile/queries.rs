use crate::store::varve::profile::ProfileRow;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

impl ProfileRow {
    queries! {
        foreign = varve;

        table = "profiles";

        names = [];

        binds = [];

        // Recurse in later
        recurse = {};
    }
}