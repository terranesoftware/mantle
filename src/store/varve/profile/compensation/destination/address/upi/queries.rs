use framboid::account::profile::compensation::destination::address::{Address, AddressKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::address::upi::UpiRow;

impl UpiRow {
    queries! {
        foreign = address;

        param = upi: &Address;

        table = "upis";

        names = ["vpa"];

        binds = [
            match upi.kind() {
                AddressKind::Upi(vpa) => vpa,
                _ => unreachable!()
            }
        ];
    }
}
