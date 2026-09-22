use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::integer::IntegerRow;

impl IntegerRow {
    queries! {
        foreign = body;
        
        param = integer: &Body;

        table = "integers";

        names = ["integer"];

        binds = [
            match integer.kind() {
                BodyKind::Integer(int) => int,
                _ => unreachable!()
            }
        ];
    }
}