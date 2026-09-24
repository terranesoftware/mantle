use framboid::addressing::body::Body;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::{BodyRow, list::{ListRow, entries::EntryRow}};

impl ListRow {
    queries! {
        foreign = body;
        
        param = list: (i64, &[Body]);
        
        table = "lists";

        names = [];

        binds = [];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            for (index, body) in list.1.iter().enumerate() {
                let body_row = Box::pin(BodyRow::insert(pool, list.0, (list.0, body))).await?;
                EntryRow::insert(pool, row.id, (index, body_row.id)).await?;
            }

            Ok(())
        };
    }
}