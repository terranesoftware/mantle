use blake3::Hash;
use framboid::{addressing::Action, keys::Key};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};
use time::UtcOffset;

use crate::store::varve::dossier::event::action::ActionRow;

impl ActionRow {
    queries! {
        foreign = event;
        
        parameters = [action: (usize, Hash, &Action)];
        
        table = "actions";
        
        names = ["event", "hash", "name", "position", "source", "time"];
        
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
}