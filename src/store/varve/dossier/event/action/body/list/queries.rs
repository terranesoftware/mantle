use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::list::ListRow;

impl ListRow {
    queries! {
        foreign = body;
        
        param = list: &Body;
        
        table = "lists";

        names = [];

        binds = [];

        recurse = {
            let list = match list.kind() {
                BodyKind::List(list) => list,
                _ => unreachable!()
            };
        };
    }
}