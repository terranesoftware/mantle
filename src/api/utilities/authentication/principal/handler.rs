use axum::{extract::Request, middleware::Next, response::Response};

use crate::api::utilities::authentication::principal::Principal;

pub async fn principal(mut request: Request, next: Next) -> Response {
    if let Some(principal) = verify().await {
        request.extensions_mut().insert(principal);
    }

    next.run(request).await
}

// PLACEHOLDER
pub async fn verify() -> Option<Principal> {
    None
}