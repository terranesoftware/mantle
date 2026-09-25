use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::profile::compensation::destination::address::pix::PixRow;

impl PixRow {
    queries! {
        foreign = address: i64;

        param = key: &str;

        table = "pixs";

        names = ["key"];

        binds = [key];
    }
}
