use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::address::iban::IbanRow;

impl IbanRow {
    queries! {
        foreign = address;

        param = iban: &str;

        table = "ibans";

        names = ["iban"];

        binds = [iban];
    }
}
