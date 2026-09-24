use framboid::account::profile::identity::name::usage::{Usage as FramboidUsage, UsageKind as FramboidUsageKind};
use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Usage(UsageKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "usage_kind", rename_all = "lowercase")]
pub enum UsageKind {
    Legal,
    Prior,
    Used
}

impl From<FramboidUsage> for Usage {
    fn from(value: FramboidUsage) -> Self {
        match value.kind() {
            FramboidUsageKind::Legal => Usage(UsageKind::Legal),
            FramboidUsageKind::Prior(_) => Usage(UsageKind::Prior),
            FramboidUsageKind::Used => Usage(UsageKind::Used)
        }
    }
}
