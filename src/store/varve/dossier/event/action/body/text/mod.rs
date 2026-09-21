pub mod queries;

use sqlx::prelude::FromRow;

/// The database representation of `BodyKind::Text`.
#[derive(FromRow)]
pub struct TextRow {
    id: i64,
    body: i64,

    text: String
}

impl TextRow {
    keys!(body);

    /// Returns a reference to the contained text.
    pub fn text(&self) -> &str {
        &self.text
    }
}