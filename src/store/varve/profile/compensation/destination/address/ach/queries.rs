use framboid::account::profile::compensation::destination::address::{Address, AddressKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::address::ach::{AchRow, account::Account};

impl AchRow {
    queries! {
        foreign = address;

        param = ach: &Address;

        table = "achs";

        names = ["account", "kind", "routing"];

        binds = [
            match ach.kind() {
                AddressKind::Ach { account, .. } => account,
                _ => unreachable!()
            },
            match ach.kind() {
                AddressKind::Ach { kind, .. } => Account::from(kind),
                _ => unreachable!()
            },
            match ach.kind() {
                AddressKind::Ach { routing, .. } => routing,
                _ => unreachable!()
            }
        ];
    }
}
