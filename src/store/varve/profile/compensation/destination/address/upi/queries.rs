use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

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
