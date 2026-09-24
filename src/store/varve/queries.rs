use framboid::{account::Varve, keys::Key};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};
use time::UtcOffset;

use crate::store::varve::{VarveRow, dossier::DossierRow, profile::ProfileRow};

impl VarveRow {
    queries! {
        param = varve: &Varve;

        table = "varves";

        names = ["account", "from"];

        binds = [varve.account().key(), varve.from().to_offset(UtcOffset::UTC)];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            for (index, dossier) in varve.dossiers().iter().enumerate() {
                DossierRow::insert(pool, row.id, (index, *dossier.0, dossier.1)).await?;
            }

            ProfileRow::insert(pool, row.id, varve.profile()).await?;

            Ok(())
        };
    }
}