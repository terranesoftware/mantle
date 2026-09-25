use blake3::Hash;
use framboid::{addressing::Action, keys::Key};
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};
use time::UtcOffset;

use crate::store::{source::SourceRow, varve::dossier::event::action::{ActionRow, body::BodyRow}};

impl ActionRow {
    queries! {
        foreign = event: i64;
        
        param = action: (usize, Hash, &Action);
        
        table = "actions";
        
        names = ["hash", "name", "position", "source", "time"];

        setup = source: async |pool| -> Result<i64, Error> {
            SourceRow::select_query_scalar(pool, "id", &format!("WHERE source = {}", action.2.source().key())).await
        };
        
        binds = [
            action.1.as_bytes(),
            action.2.name(),
            action.0 as i64,
            source,
            action.2.time().to_offset(UtcOffset::UTC)
        ];

        recurse = async |pool, row: &Self| {
            BodyRow::insert(pool, row.id, (row.id, action.2.body())).await
        };
    }
}