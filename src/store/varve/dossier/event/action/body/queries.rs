use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::{BodyRow, kind::Body};

impl BodyRow {
    queries! {
        foreign = action;
        
        parameters = [body: &framboid::addressing::body::Body];
        
        table = "bodies";
        
        names = ["action", "kind"];
        
        binds = [action, Body::from(body)];

        // Complete this
        recurse = {};
    }
}