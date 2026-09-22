use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::boolean::BooleanRow;

impl BooleanRow {
    queries! {
        foreign = body;

        param = boolean: &Body;

        table = "booleans";

        names = ["boolean"];

        binds = [
            match boolean.kind() {
                BodyKind::Boolean(bool) => bool,
                _ => unreachable!()
            }
        ];
    }
}