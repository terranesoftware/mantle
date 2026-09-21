use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, query_as};

use crate::store::varve::dossier::event::action::body::map::MapRow;

impl MapRow {
    insert! {
        parameters = [body: i64, map: &Body];

        setup = {
            let map = match map.kind() {
                BodyKind::Map(map) => map,
                _ => unreachable!()
            };
        };

        row = MapRow;

        table = "maps";

        columns = [1];

        binds = [body];

        // Complete this
        recurse = {};
    }

    delete!("maps");

    select!("maps");

    update! {
        parameters = [body: Option<i64>];

        table = "maps";

        names = ["body"];

        numbers = [2];

        binds = [body];
    }
}