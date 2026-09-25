use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::profile::compensation::destination::address::iban::IbanRow;

impl IbanRow {
    queries! {
        foreign = address: i64;

        param = iban: &str;

        table = "ibans";

        names = ["iban"];

        binds = [iban];
    }
}
