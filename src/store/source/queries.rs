use framboid::keys::{Key, source::SourceKey};
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::source::SourceRow;

impl SourceRow {
    queries! {
        param = source: &SourceKey;

        table = "sources";

        names = ["source"];

        binds = [source.key()];
    }
}