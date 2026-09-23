use framboid::account::profile::engagement::Engagement;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::{
    engagement::{EngagementRow, arrangement::Arrangement, employment::Employment},
    location::Location,
    period::Period
};

impl EngagementRow {
    queries! {
        foreign = profile;

        param = engagement: &Engagement;

        table = "engagements";

        names = ["arrangement", "employer", "employment", "location", "period", "title"];

        binds = [
            Arrangement::from(&engagement.arrangement()),
            engagement.employer(),
            Employment::from(&engagement.employment()),
            engagement.location().map(Location::from),
            Period::from(&engagement.period()),
            engagement.title()
        ];
    }
}
