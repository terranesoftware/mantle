use crate::store::varve::dossier::event::action::body::map::entries::EntryRow;
use sqlx::{Error, PgPool, query_as};

impl EntryRow {
    insert! {
        parameters = [map: i64, entry: (usize, i64, &str)];

        row = EntryRow;

        table = "map.entries";

        columns = [1, 2, 3, 4];

        binds = [map, entry.1, entry.2, entry.0 as i64];
    }

    delete!("map.entries");

    select!("map.entries");

    update! {
        parameters = [map: Option<i64>, entry: (usize, i64, &str)];

        table = "map.entries";

        names = ["map", "contains", "name", "position"];

        numbers = [2, 3, 4, 5];

        binds = [map, entry.1, entry.2, entry.0 as i64];
    }
}