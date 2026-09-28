use axum::{extract::FromRequestParts, http::StatusCode};

use crate::api::utilities::authentication::principal::Principal;

#[derive(Clone, Copy)]
pub struct Anonymous;

impl<S> FromRequestParts<S> for Anonymous
where
    S: Send + Sync
{
    type Rejection = StatusCode;
    
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection>
    {
        if let Some(_) = parts.extensions.get::<Principal>() {
            Err(StatusCode::CONFLICT)
        }
        else {
            Ok(Anonymous)
        }
    }
}