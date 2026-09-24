use blake3::Hash;
use framboid::{addressing::Action, keys::Key};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};
use time::UtcOffset;

use crate::store::varve::dossier::event::action::{ActionRow, body::BodyRow};

impl ActionRow {
    queries! {
        foreign = event;
        
        param = action: (usize, Hash, &Action);
        
        table = "actions";
        
        names = ["hash", "name", "position", "source", "time"];
        
        binds = [
            action.1.as_bytes(),
            action.2.name(),
            action.0 as i64,
            action.2.source().key(),
            action.2.time().to_offset(UtcOffset::UTC)
        ];

        recurse = async |pool, row: &Self| {
            BodyRow::insert(pool, row.id, (row.id, action.2.body())).await
        };
    }
}