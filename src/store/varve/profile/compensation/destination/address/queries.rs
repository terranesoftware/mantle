use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::address::{AddressRow, kind::Address};

impl AddressRow {
    queries! {
        foreign = destination;

        param = kind: Address;

        table = "destination.addresses";

        names = ["kind"];

        binds = [Address::from(kind)];

        recurse = {};
    }
}