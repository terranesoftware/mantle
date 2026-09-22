use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::map::MapRow;

impl MapRow {
    queries! {
        foreign = body;
        
        param = map: &Body;
        
        table = "maps";

        names = [];

        binds = [];

        recurse = {
            let map = match map.kind() {
                BodyKind::Map(map) => map,
                _ => unreachable!()
            };
        };
    }
}