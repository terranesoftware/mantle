use framboid::addressing::body::{Body as FramboidBody, BodyKind as FramboidBodyKind};
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::dossier::event::action::body::{BodyRow, boolean::BooleanRow, handle::HandleRow, integer::IntegerRow, kind::Body, list::ListRow, map::MapRow, text::TextRow};

impl BodyRow {
    queries! {
        foreign = action: i64;
        
        param = body: (i64, &FramboidBody);
        
        table = "bodies";
        
        names = ["kind"];
        
        binds = [Body::from(body.1)];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            match body.1.kind() {
                FramboidBodyKind::Boolean(boolean) => _ = BooleanRow::insert(pool, row.id, *boolean).await?,
                FramboidBodyKind::Handle(handle) => _ = HandleRow::insert(pool, row.id, handle.as_str()).await?,
                FramboidBodyKind::Integer(integer) => _ = IntegerRow::insert(pool, row.id, *integer).await?,
                FramboidBodyKind::List(list) => _ = ListRow::insert(pool, row.id, (body.0, list)).await?,
                FramboidBodyKind::Map(map) => _ = MapRow::insert(pool, row.id, (body.0, map)).await?,
                FramboidBodyKind::Text(text) => _ = TextRow::insert(pool, row.id, text.as_str()).await?
            };

            Ok(())
        };
    }
}