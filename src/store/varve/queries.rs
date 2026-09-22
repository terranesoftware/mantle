use framboid::{account::Varve, keys::Key};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};
use time::UtcOffset;

use crate::store::varve::VarveRow;

impl VarveRow {
    queries! {
        parameters = [varve: &Varve];

        table = "varves";

        names = ["account", "from"];

        binds = [varve.account().key(), varve.from().to_offset(UtcOffset::UTC)];
    }
}