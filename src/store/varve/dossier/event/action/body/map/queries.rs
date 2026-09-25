use framboid::addressing::body::Body;
use indexmap::IndexMap;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::dossier::event::action::body::{BodyRow, map::{MapRow, entries::EntryRow}};

impl MapRow {
    queries! {
        foreign = body: i64;
        
        param = map: (i64, &IndexMap<String, Body>);
        
        table = "maps";

        names = [];

        binds = [];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            for (index, body) in map.1.iter().enumerate() {
                let body_row = Box::pin(BodyRow::insert(pool, map.0, (map.0, body.1))).await?;
                EntryRow::insert(pool, row.id, (index, body_row.id, body.0.as_str())).await?;
            }

            Ok(())
        };
    }
}