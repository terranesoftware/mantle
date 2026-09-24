use framboid::account::profile::identity::address::Address;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::{
    identity::address::AddressRow,
    location::Location
};

impl AddressRow {
    queries! {
        foreign = identity;

        param = address: &Address;

        table = "identity.addresses";

        names = ["lines", "location", "postcode"];

        binds = [
            address.lines(),
            Location::from(address.location()),
            address.postcode()
        ];
    }
}
