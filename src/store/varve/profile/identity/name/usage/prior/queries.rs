use framboid::account::profile::period::Period as FramboidPeriod;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

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
