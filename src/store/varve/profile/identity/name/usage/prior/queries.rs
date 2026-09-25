use framboid::account::profile::period::Period as FramboidPeriod;
use sqlx::{Decode, Error, PgPool, Postgres, QueryBuilder, postgres::PgRow, query_as, Type};

use crate::store::varve::profile::{
    identity::name::usage::prior::PriorRow,
    period::Period
};

impl PriorRow {
    queries! {
        foreign = usage;

        param = period: FramboidPeriod;

        table = "priors";

        names = ["period"];

        binds = [Period::from(period)];
    }
}
