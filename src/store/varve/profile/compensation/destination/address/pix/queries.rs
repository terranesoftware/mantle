use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::compensation::destination::address::pix::PixRow;

impl PixRow {
    queries! {
        foreign = address;

        param = key: &str;

        table = "pixs";

        names = ["key"];

        binds = [key];
    }
}
