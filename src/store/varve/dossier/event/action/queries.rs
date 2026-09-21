use blake3::Hash;
use framboid::{addressing::Action, keys::Key};
use sqlx::{Error, PgPool, query_as};
use time::UtcOffset;

use crate::store::varve::dossier::event::action::ActionRow;

impl ActionRow {
    insert! {
        parameters = [event: i64, action: (usize, Hash, &Action)];

        row = ActionRow;

        table = "actions";

        columns = [1, 2, 3, 4, 5, 6];

        binds = [
            event,
            action.1.as_bytes(),
            action.2.name(),
            action.0 as i64,
            action.2.source().key(),
            action.2.time().to_offset(UtcOffset::UTC)
        ];

        // Complete this
        recurse = {};
    }

    delete!("actions");

    select!("actions");
    
    update! {
        parameters = [event: Option<i64>, action: (usize, Hash, &Action)];

        table = "actions";

        names = ["event", "hash", "name", "position", "source", "time"];

        numbers = [2, 3, 4, 5, 6, 7];

        binds = [
            event,
            action.1.as_bytes(),
            action.2.name(),
            action.0 as i64,
            action.2.source().key(),
            action.2.time().to_offset(UtcOffset::UTC)
        ];
    }
}