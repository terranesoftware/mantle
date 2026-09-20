use blake3::Hash;
use framboid::{addressing::Action, keys::Key};
use sqlx::{Error, PgPool, query_as};
use time::UtcOffset;

use crate::store::varve::dossier::events::action::ActionRow;

impl ActionRow {
    pub async fn insert(
        pool: &PgPool,
        event: i64,
        action: (usize, Hash, &Action)
    ) -> Result<Self, Error> {
        let row: ActionRow = query_as("INSERT INTO actions VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(event)
            .bind(action.1.as_bytes())
            .bind(action.2.name())
            .bind(action.0 as i64)
            .bind(action.2.source().key())
            .bind(action.2.time().to_offset(UtcOffset::UTC))
            .fetch_one(pool)
            .await?;

        // Recurse here

        Ok(row)
    }

    pub async fn delete(
        pool: &PgPool,
        id: i64
    ) -> Result<Self, Error> {
        query_as("DELETE FROM actions WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn select(
        pool: &PgPool,
        id: i64
    ) -> Result<Self, Error> {
        query_as("SELECT * FROM actions WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn update(
        pool: &PgPool,
        id: i64,
        event: Option<i64>,
        action: (usize, Hash, &Action)
    ) -> Result<Self, Error> {
        query_as("UPDATE actions SET (event, hash, name, position, source, time) = (COALESCE ($1, event), $2, $3, $4, $5, $6) WHERE id = $7 RETURNING *")
            .bind(event)
            .bind(action.1.as_bytes())
            .bind(action.2.name())
            .bind(action.0 as i64)
            .bind(action.2.source().key())
            .bind(action.2.time().to_offset(UtcOffset::UTC))
            .bind(id)
            .fetch_one(pool)
            .await
    }
}