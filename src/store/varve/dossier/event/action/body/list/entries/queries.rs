use crate::store::varve::dossier::event::action::body::list::entries::EntryRow;
use sqlx::{Error, PgPool, query_as};

impl EntryRow {
    insert! {
        parameters = [list: i64, entry: (usize, i64)];

        row = EntryRow;

        table = "list.entries";

        columns = [1, 2, 3];

        binds = [list, entry.1, entry.0 as i64];
    }

    delete!("list.entries");

    select!("list.entries");

    update! {
        parameters = [list: Option<i64>, entry: (usize, i64)];

        table = "list.entries";

        names = ["list", "contains", "position"];

        numbers = [2, 3, 4];

        binds = [list, entry.1, entry.0 as i64];
    }
}