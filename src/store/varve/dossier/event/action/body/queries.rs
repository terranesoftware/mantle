use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::dossier::event::action::body::{BodyRow, kind::Body};

impl BodyRow {
    queries! {
        foreign = action;
        
        param = body: &framboid::addressing::body::Body;
        
        table = "bodies";
        
        names = ["kind"];
        
        binds = [Body::from(body)];

        // Complete this
        recurse = {};
    }
}