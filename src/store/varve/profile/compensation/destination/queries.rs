use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::{DestinationRow, kind::Destination};

impl DestinationRow {
    queries! {
        foreign = compensation;

        param = kind: &framboid::account::profile::compensation::destination::Destination;

        table = "destinations";

        names = ["kind"];

        binds = [Destination::from(kind)];

        recurse = {};
    }
}