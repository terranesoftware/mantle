pub mod boolean;
pub mod handle;
pub mod integer;
pub mod kind;
pub mod list;
pub mod map;
pub mod queries;
pub mod text;

use sqlx::prelude::FromRow;

use crate::store::varve::dossier::event::action::body::kind::Body;

/// The database representation of a `Body`.
#[derive(FromRow)]
pub struct BodyRow {
    id: i64,
    action: i64,

    kind: Body
}

impl BodyRow {
    keys!(action);

    /// Returns a copy of the contained `Body`.
    pub fn kind(&self) -> Body {
        self.kind
    }
}