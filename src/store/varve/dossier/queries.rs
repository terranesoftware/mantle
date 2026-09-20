use blake3::Hash;
use framboid::account::dossier::Dossier;
use sqlx::{Error, PgPool, query_as};
use time::UtcOffset;

use crate::store::varve::dossier::DossierRow;

impl DossierRow {
    pub async fn insert(
        pool: &PgPool,
        varve: i64,
        dossier: (usize, Hash, &Dossier)
    ) -> Result<Self, Error> {
        let row: DossierRow = query_as("INSERT INTO dossiers VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(varve)
            .bind(dossier.2.from().to_offset(UtcOffset::UTC))
            .bind(dossier.1.as_bytes())
            .bind(dossier.0 as i64)
            .bind(dossier.2.to().to_offset(UtcOffset::UTC))
            .fetch_one(pool)
            .await?;

        // Recurse here

        Ok(row)
    }

    pub async fn delete(
        pool: &PgPool,
        id: i64
    ) -> Result<Self, Error> {
        query_as("DELETE FROM dossiers WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn select(
        pool: &PgPool,
        id: i64
    ) -> Result<Self, Error> {
        query_as("SELECT * FROM dossiers WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn update(
        pool: &PgPool,
        id: i64,
        varve: Option<i64>,
        dossier: (usize, Hash, &Dossier)
    ) -> Result<Self, Error> {
        query_as("UPDATE dossiers SET (varve, from, hash, position, to) = (COALESCE ($1, varve), $2, $3, $4, $5) WHERE id = $6 RETURNING *")
            .bind(varve)
            .bind(dossier.2.from().to_offset(UtcOffset::UTC))
            .bind(dossier.1.as_bytes())
            .bind(dossier.0 as i64)
            .bind(dossier.2.to().to_offset(UtcOffset::UTC))
            .bind(id)
            .fetch_one(pool)
            .await
    }
}