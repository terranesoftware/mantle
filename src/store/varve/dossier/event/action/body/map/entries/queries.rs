use crate::store::varve::dossier::event::action::body::map::entries::EntryRow;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

impl EntryRow {
    queries! {
        foreign = map;

        param = entry: (usize, i64, &str);

        table = "map.entries";

        names = ["contains", "name", "position"];

        binds = [entry.1, entry.2, entry.0 as i64];
    }
}