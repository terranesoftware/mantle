use blake3::Hash;
use framboid::account::dossier::Dossier;
use sqlx::{Error, PgPool, query_as};
use time::UtcOffset;

use crate::store::varve::dossier::DossierRow;

impl DossierRow {
    insert! {
        parameters = [varve: i64, dossier: (usize, Hash, &Dossier)];

        row = DossierRow;

        table = "dossiers";

        columns = [1, 2, 3, 4, 5];

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

    delete!("dossiers");

    select!("dossiers");

    update! {
        parameters = [varve: Option<i64>, dossier: (usize, Hash, &Dossier)];

        table = "dossiers";

        names = ["varve", "from", "hash", "position", "to"];

        numbers = [2, 3, 4, 5, 6];

        binds = [
            varve,
            dossier.2.from().to_offset(UtcOffset::UTC),
            dossier.1.as_bytes(),
            dossier.0 as i64,
            dossier.2.to().to_offset(UtcOffset::UTC)
        ];
    }
}