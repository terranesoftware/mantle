use framboid::account::profile::engagement::employment::{Employment as FramboidEmployment, EmploymentKind as FramboidEmploymentKind};
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

impl From<FramboidEmployment> for Employment {
    fn from(value: FramboidEmployment) -> Self {
        match value.kind() {
            FramboidEmploymentKind::Apprenticeship => Employment(EmploymentKind::Apprenticeship),
            FramboidEmploymentKind::Contract => Employment(EmploymentKind::Contract),
            FramboidEmploymentKind::Internship => Employment(EmploymentKind::Internship),
            FramboidEmploymentKind::FullTime => Employment(EmploymentKind::FullTime),
            FramboidEmploymentKind::PartTime => Employment(EmploymentKind::PartTime)
        }
    }
}
