use framboid::account::profile::credentials::status::{Status as FramboidStatus, kind::StatusKind as FramboidStatusKind};
use sqlx::prelude::Type;

#[derive(Clone, Copy, Type)]
#[sqlx(transparent)]
pub struct Status(StatusKind);

#[derive(Clone, Copy, Type)]
#[sqlx(type_name = "status", rename_all = "snake_case")]
pub enum StatusKind {
    Completed,
    Enrolled,
    DroppedOut,
    Transferred,
    Withdrawn
}

impl From<FramboidStatus> for Status {
    fn from(value: FramboidStatus) -> Self {
        match value.kind() {
            FramboidStatusKind::Completed => Status(StatusKind::Completed),
            FramboidStatusKind::Enrolled => Status(StatusKind::Enrolled),
            FramboidStatusKind::DroppedOut => Status(StatusKind::DroppedOut),
            FramboidStatusKind::Transferred => Status(StatusKind::Transferred),
            FramboidStatusKind::Withdrawn => Status(StatusKind::Withdrawn)
        }
    }
}
