pub mod authorization;
pub mod compensation;
pub mod credential;
pub mod identity;
pub mod location;
pub mod period;
pub mod queries;

use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct ProfileRow {
    id: i64,
    varve: i64
}

impl ProfileRow {
    keys!(varve);
}