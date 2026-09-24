use framboid::account::profile::identity::name::usage::{Usage, UsageKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::{
    identity::name::usage::prior::PriorRow,
    period::Period
};

impl PriorRow {
    queries! {
        foreign = usage;

        param = prior: Usage;

        table = "priors";

        names = ["period"];

        binds = [
            match prior.kind() {
                UsageKind::Prior(period) => Period::from(&period),
                _ => unreachable!()
            }
        ];
    }
}
