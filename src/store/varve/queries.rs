use framboid::{account::Varve, keys::Key};
use sqlx::{Error, PgPool, query_as};
use time::UtcOffset;

use crate::store::varve::VarveRow;

impl VarveRow {
    insert! {
        parameters = [varve: &Varve];

        row = VarveRow;

        table = "varves";

        columns = [1, 2];

        binds = [
            varve.account().key(),
            varve.from().to_offset(UtcOffset::UTC)
        ];
    }
    
    delete!("varves");
    
    select!("varves");

    update! {
        parameters = [varve: &Varve];

        table = "varves";

        names = ["account", "from"];

        numbers = [2, 3];

        binds = [
            varve.account().key(),
            varve.from().to_offset(UtcOffset::UTC)
        ];
    }
}