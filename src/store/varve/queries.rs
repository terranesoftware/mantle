use framboid::{account::Varve, keys::Key};
use sqlx::{Error, PgPool, query_as};
use time::UtcOffset;

use crate::store::varve::VarveRow;

impl VarveRow {
    pub async fn insert(
        pool: &PgPool,
        varve: &Varve
    ) -> Result<Self, Error> {
        let row: VarveRow = query_as("INSERT INTO varves VALUES ($1, $2) RETURNING *")
            .bind(varve.account().key())
            .bind(varve.from().to_offset(UtcOffset::UTC))
            .fetch_one(pool)
            .await?;

        // Recurse here

        Ok(row)
    }

    pub async fn delete(
        pool: &PgPool,
        id: i64
    ) -> Result<Self, Error> {
        query_as("DELETE FROM varves WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn select(
        pool: &PgPool,
        id: i64
    ) -> Result<Self, Error> {
        query_as("SELECT * FROM varves WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn update(
        pool: &PgPool,
        varve: &Varve
    ) -> Result<Self, Error> {
        query_as("UPDATE varves SET (account, from) = ($1, $2) WHERE id = $3 RETURNING *")
            .bind(varve.account().key())
            .bind(varve.from().to_offset(UtcOffset::UTC))
            .fetch_one(pool)
            .await
    }
}