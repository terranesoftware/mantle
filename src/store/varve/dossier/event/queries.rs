use blake3::Hash;
use framboid::account::dossier::event::Event;
use sqlx::{Error, PgPool, query_as};
use time::UtcOffset;

use crate::store::varve::dossier::event::EventRow;

impl EventRow {
    insert! {
        parameters = [dossier: i64, event: (usize, Hash, &Event)];

        row = EventRow;

        table = "events";

        columns = [1, 2, 3, 4, 5];

        binds = [
            dossier,
            event.2.from().to_offset(UtcOffset::UTC),
            event.1.as_bytes(),
            event.0 as i64,
            event.2.to().to_offset(UtcOffset::UTC)
        ];

        // Complete this
        recurse = {};
    }

    delete!("events");

    select!("events");

    update! {
        parameters = [dossier: Option<i64>, event: (usize, Hash, &Event)];

        table = "events";

        names = ["dossier", "from", "hash", "position", "to"];

        numbers = [2, 3, 4, 5, 6];

        binds = [
            dossier,
            event.2.from().to_offset(UtcOffset::UTC),
            event.1.as_bytes(),
            event.0 as i64,
            event.2.to().to_offset(UtcOffset::UTC)
        ];
    }
}