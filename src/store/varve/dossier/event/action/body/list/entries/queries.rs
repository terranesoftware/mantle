use crate::store::varve::dossier::event::action::body::list::entries::EntryRow;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

impl EntryRow {
    queries! {
        foreign = list;
        
        param = entry: (usize, i64);

        table = "list.entries";

        names = ["contains", "position"];

        binds = [entry.1, entry.0 as i64];
    }
}