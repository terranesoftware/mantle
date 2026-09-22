use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::handle::HandleRow;

impl HandleRow {
    queries! {
        foreign = body;

        param = handle: &Body;

        table = "handles";

        names = ["handle"];

        binds = [
            match handle.kind() {
                BodyKind::Handle(str) => str,
                _ => unreachable!()
            }
        ];
    }
}