use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, query_as};

use crate::store::varve::dossier::event::action::body::integer::IntegerRow;

impl IntegerRow {
    insert! {
        parameters = [body: i64, integer: Body];

        setup = {
            let integer = match integer.kind() {
                BodyKind::Integer(int) => int,
                _ => unreachable!()
            };
        };

        row = IntegerRow;

        table = "integers";

        columns = [1, 2];

        binds = [body, integer];
    }

    delete!("integers");

    select!("integers");

    update! {
        parameters = [body: Option<i64>, integer: i64];

        table = "integers";

        names = ["body", "integer"];

        numbers = [2, 3];

        binds = [body, integer];
    }
}