use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::address::upi::UpiRow;

impl UpiRow {
    queries! {
        foreign = address;

        param = upi: &str;

        table = "upis";

        names = ["vpa"];

        binds = [upi];
    }
}
