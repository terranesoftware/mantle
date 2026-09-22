use blake3::Hash;
use framboid::account::dossier::event::Event;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};
use time::UtcOffset;

use crate::store::varve::dossier::event::EventRow;

impl EventRow {
    queries! {
        foreign = dossier;
        
        param = event: (usize, Hash, &Event);
        
        table = "events";

        names = ["from", "hash", "position", "to"];
        
        binds = [
            event.2.from().to_offset(UtcOffset::UTC),
            event.1.as_bytes(),
            event.0 as i64,
            event.2.to().to_offset(UtcOffset::UTC)
        ];

        // Complete this
        recurse = {};
    }
}