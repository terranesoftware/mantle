use blake3::Hash;
use framboid::account::dossier::Dossier;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};
use time::UtcOffset;

use crate::store::varve::dossier::{DossierRow, event::EventRow};

impl DossierRow {
    queries! {
        foreign = varve: i64;
        
        param = dossier: (usize, Hash, &Dossier);
        
        table = "dossiers";
        
        names = ["since", "hash", "position", "to"];

        binds = [
            dossier.2.since().to_offset(UtcOffset::UTC),
            dossier.1.as_bytes(),
            dossier.0 as i64,
            dossier.2.until().to_offset(UtcOffset::UTC)
        ];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            for (index, event) in dossier.2.events().iter().enumerate() {
                EventRow::insert(pool, row.id, (index, *event.0, event.1)).await?;
            }

            Ok(())
        };
    }
}