use framboid::account::profile::compensation::destination::address::{Address, AddressKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::address::iban::IbanRow;

impl IbanRow {
    queries! {
        foreign = address;

        param = iban: &Address;

        table = "ibans";

        names = ["iban"];

        binds = [
            match iban.kind() {
                AddressKind::Iban(iban) => iban,
                _ => unreachable!()
            }
        ];
    }
}
