use sqlx::{Error, PgPool, raw_sql};

pub async fn initialize(pool: &PgPool) -> Result<(), Error> {
    raw_sql(include_str!("../../sql/schema.sql"))
        .execute(pool)
        .await?;

    Ok(())
}