use crate::store::varve::dossier::event::action::body::map::entries::EntryRow;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

impl EntryRow {
    queries! {
        foreign = map: i64;

        param = entry: (usize, i64, &str);

        table = "map.entries";

        names = ["contains", "name", "position"];

        binds = [entry.1, entry.2, entry.0 as i64];
    }
}