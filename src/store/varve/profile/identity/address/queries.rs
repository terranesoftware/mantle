use framboid::account::profile::identity::address::Address;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

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
