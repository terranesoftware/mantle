pub mod kind;
pub mod queries;

use sqlx::prelude::FromRow;

use crate::store::varve::dossier::event::action::body::kind::Body;

/// The database representation of a `Body`.
#[derive(FromRow)]
pub struct BodyRow {
    id: i64,
    body: i64,

    kind: Body
}

impl BodyRow {
    /// Returns a copy of the contained primary key.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns a copy of the contained foreign key.
    pub fn body(&self) -> i64 {
        self.body
    }

    /// Returns a copy of the contained `Body`.
    pub fn kind(&self) -> Body {
        self.kind
    }
}