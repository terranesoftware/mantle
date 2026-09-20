use sqlx::{Error, PgPool, query_as};

use crate::store::varve::dossier::event::action::body::{BodyRow, kind::Body};

impl BodyRow {
    pub async fn insert(
        pool: &PgPool,
        action: i64,
        body: framboid::addressing::body::Body
    ) -> Result<Self, Error> {
        let row: BodyRow = query_as("INSERT INTO bodies VALUES ($1, $2) RETURNING *")
            .bind(action)
            .bind(Body::from(body))
            .fetch_one(pool)
            .await?;

        // Recurse here

        Ok(row)
    }

    pub async fn delete(
        pool: &PgPool,
        id: i64
    ) -> Result<Self, Error> {
        query_as("DELETE FROM bodies WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn select(
        pool: &PgPool,
        id: i64
    ) -> Result<Self, Error> {
        query_as("SELECT * FROM bodies WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn update(
        pool: &PgPool,
        id: i64,
        action: Option<i64>,
        body: framboid::addressing::body::Body
    ) -> Result<Self, Error> {
        query_as("UPDATE bodies SET (action, kind) = (COALESCE ($1, action), $2) WHERE id = $3 RETURNING *")
            .bind(action)
            .bind(Body::from(body))
            .bind(id)
            .fetch_one(pool)
            .await
    }
}