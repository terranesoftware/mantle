use blake3::Hash;
use framboid::account::dossier::event::Event;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};
use time::UtcOffset;

use crate::store::varve::dossier::event::{EventRow, action::ActionRow};

impl EventRow {
    queries! {
        foreign = dossier: i64;
        
        param = event: (usize, Hash, &Event);
        
        table = "events";

        names = ["from", "hash", "position", "to"];
        
        binds = [
            event.2.from().to_offset(UtcOffset::UTC),
            event.1.as_bytes(),
            event.0 as i64,
            event.2.to().to_offset(UtcOffset::UTC)
        ];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            for (index, action) in event.2.actions().iter().enumerate() {
                ActionRow::insert(pool, row.id, (index, *action.0, action.1)).await?;
            }

            Ok(())
        };
    }
}