use blake3::Hash;
use framboid::account::dossier::Dossier;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};
use time::UtcOffset;

use crate::store::varve::dossier::DossierRow;

impl DossierRow {
    queries! {
        foreign = varve;
        
        parameters = [dossier: (usize, Hash, &Dossier)];
        
        table = "dossiers";
        
        names = ["varve", "from", "hash", "position", "to"];

        binds = [
            varve,
            dossier.2.from().to_offset(UtcOffset::UTC),
            dossier.1.as_bytes(),
            dossier.0 as i64,
            dossier.2.to().to_offset(UtcOffset::UTC)
        ];

        // Complete this
        recurse = {};
    }
}