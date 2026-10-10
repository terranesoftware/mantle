use framboid::{account::Varve, keys::Key};
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};
use time::UtcOffset;

use crate::store::{account::AccountRow, varve::{VarveRow, dossier::DossierRow, profile::ProfileRow}};

impl VarveRow {
    queries! {
        param = varve: &Varve;

        table = "varves";

        names = ["account", "since"];

        setup = account: async |pool| -> Result<i64, Error> {
            AccountRow::select_query_scalar(pool, "id", &format!("WHERE account = '{}'", varve.account().key())).await
        };

        binds = [account, varve.since().to_offset(UtcOffset::UTC)];

        recurse = async |pool, row: &Self| -> Result<(), Error> {
            for (index, dossier) in varve.dossiers().iter().enumerate() {
                DossierRow::insert(pool, row.id, (index, *dossier.0, dossier.1)).await?;
            }

            ProfileRow::insert(pool, row.id, varve.profile()).await?;

            Ok(())
        };
    }
}