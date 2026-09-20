use blake3::Hash;
use framboid::account::dossier::event::Event;
use sqlx::{Error, PgPool, query_as};
use time::UtcOffset;

use crate::store::varve::dossier::events::EventRow;

impl EventRow {
    pub async fn insert(
        pool: &PgPool,
        dossier: i64,
        event: (usize, Hash, &Event)
    ) -> Result<Self, Error> {
        let row: EventRow = query_as("INSERT INTO events VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(dossier)
            .bind(event.2.from().to_offset(UtcOffset::UTC))
            .bind(event.1.as_bytes())
            .bind(event.0 as i64)
            .bind(event.2.to().to_offset(UtcOffset::UTC))
            .fetch_one(pool)
            .await?;

        // Recurse here
        
        Ok(row)
    }

    pub async fn delete(
        pool: &PgPool,
        id: i64
    ) -> Result<Self, Error> {
        query_as("DELETE FROM events WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn select(
        pool: &PgPool,
        id: i64
    ) -> Result<Self, Error> {
        query_as("SELECT * FROM events WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn update(
        pool: &PgPool,
        id: i64,
        dossier: Option<i64>,
        event: (usize, Hash, &Event)
    ) -> Result<Self, Error> {
        query_as("UPDATE events SET (dossier, from, hash, position, to) = (COALESCE ($1, dossier), $2, $3, $4, $5) WHERE id = $6 RETURNING *")
            .bind(dossier)
            .bind(event.2.from().to_offset(UtcOffset::UTC))
            .bind(event.1.as_bytes())
            .bind(event.0 as i64)
            .bind(event.2.to().to_offset(UtcOffset::UTC))
            .bind(id)
            .fetch_one(pool)
            .await
    }
}