use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Employment(EmploymentKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "employment", rename_all = "snake_case")]
pub enum EmploymentKind {
    Apprenticeship,
    Contract,
    Internship,
    FullTime,
    PartTime
}

impl From<&framboid::account::profile::engagement::employment::Employment> for Employment {
    fn from(value: &framboid::account::profile::engagement::employment::Employment) -> Self {
        match value.kind() {
            framboid::account::profile::engagement::employment::EmploymentKind::Apprenticeship => Employment(EmploymentKind::Apprenticeship),
            framboid::account::profile::engagement::employment::EmploymentKind::Contract => Employment(EmploymentKind::Contract),
            framboid::account::profile::engagement::employment::EmploymentKind::Internship => Employment(EmploymentKind::Internship),
            framboid::account::profile::engagement::employment::EmploymentKind::FullTime => Employment(EmploymentKind::FullTime),
            framboid::account::profile::engagement::employment::EmploymentKind::PartTime => Employment(EmploymentKind::PartTime)
        }
    }
}
