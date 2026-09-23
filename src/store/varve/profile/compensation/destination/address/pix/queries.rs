use framboid::account::profile::compensation::destination::address::{Address, AddressKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::address::pix::PixRow;

impl PixRow {
    queries! {
        foreign = address;

        param = pix: &Address;

        table = "pixs";

        names = ["identifier"];

        binds = [
            match pix.kind() {
                AddressKind::Pix(identifier) => identifier,
                _ => unreachable!()
            }
        ];
    }
}
