use sqlx::{Error, PgPool, query_as};

use crate::store::varve::dossier::event::action::body::{BodyRow, kind::Body};

impl BodyRow {
    insert! {
        parameters = [action: i64, body: framboid::addressing::body::Body];

        row = BodyRow;

        table = "bodies";

        columns = [1, 2];

        binds = [action, Body::from(body)];

        // Complete this
        recurse = {};
    }

    delete!("bodies");

    select!("bodies");

    update! {
        parameters = [action: Option<i64>, body: framboid::addressing::body::Body];

        table = "bodies";

        names = ["action", "kind"];

        numbers = [2, 3];

        binds = [action, Body::from(body)];
    }
}