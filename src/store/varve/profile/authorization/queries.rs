use crate::store::varve::profile::authorization::AuthorizationRow;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

impl AuthorizationRow {
    queries! {
        foreign = profile;

        table = "authorizations";

        names = [];

        binds = [];

        // Recurse in later
        recurse = {};
    }
}