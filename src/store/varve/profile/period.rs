use sqlx::prelude::Type;
use time::Date;

#[derive(Type)]
#[sqlx(type_name = "period")]
pub struct Period {
    start: Date,
    end: Option<Date>
}

impl From<&framboid::account::profile::period::Period> for Period {
    fn from(value: &framboid::account::profile::period::Period) -> Self {
        Self {
            start: value.start(),
            end: value.end()
        }
    }
}