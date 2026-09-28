use axum::{extract::Request, middleware::Next, response::Response};
use framboid::keys::account::AccountKey;

use crate::organization::key::OrganizationKey;

#[derive(Clone)]
pub struct Principal {
    account: AccountKey,
    organization: Option<OrganizationKey>
}

pub async fn principal(mut request: Request, next: Next) -> Response {
    if let Some(principal) = authenticate().await {
        request.extensions_mut().insert(principal);
    }

    next.run(request).await
}

// PLACEHOLDER
pub async fn authenticate() -> Option<Principal> {
    None
}