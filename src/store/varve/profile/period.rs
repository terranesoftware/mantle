use framboid::account::profile::period::Period as FramboidPeriod;
use sqlx::prelude::Type;
use time::Date;

#[derive(Type)]
#[sqlx(type_name = "period")]
pub struct Period {
    start: Date,
    end: Option<Date>
}

impl From<FramboidPeriod> for Period {
    fn from(value: FramboidPeriod) -> Self {
        Self {
            start: value.start(),
            end: value.end()
        }
    }
}