use framboid::keys::{Key, source::SourceKey};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::source::SourceRow;

impl SourceRow {
    queries! {
        param = source: &SourceKey;

        table = "sources";

        names = ["source"];

        binds = [source.key()];
    }
}