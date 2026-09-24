use framboid::account::profile::compensation::destination::address::account::Account as FramboidAccount;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::address::ach::{AchRow, account::Account};

impl AchRow {
    queries! {
        foreign = address;

        param = ach: (&str, FramboidAccount, &str);

        table = "achs";

        names = ["account", "kind", "routing"];

        binds = [ach.0, Account::from(ach.1), ach.2];
    }
}
