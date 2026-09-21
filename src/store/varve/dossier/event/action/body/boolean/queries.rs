use framboid::addressing::body::{Body, BodyKind};
use sqlx::{Error, PgPool, query_as};

use crate::store::varve::dossier::event::action::body::boolean::BooleanRow;

impl BooleanRow {
    pub async fn insert(
        pool: &PgPool,
        body: i64,
        boolean: Body
    ) -> Result<Self, Error> {
        // Decide how to error here later
        let boolean = match boolean.kind() {
            BodyKind::Boolean(bool) => bool,
            _ => unreachable!()
        };
        
        query_as("INSERT INTO booleans VALUES ($1, $2) RETURNING *")
            .bind(body)
            .bind(boolean)
            .fetch_one(pool)
            .await
    }
}