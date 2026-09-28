use axum::{body::Bytes, extract::FromRequest, http::StatusCode, response::IntoResponse};
use bitcode::{deserialize, serialize};
use serde::{Serialize, de::DeserializeOwned};

use crate::api::utilities::bitcode::rejection::BitcodeRejection;

pub mod rejection;

pub struct Bitcode<T>(pub T);

impl<S: Send + Sync, T: DeserializeOwned> FromRequest<S> for Bitcode<T> {
    type Rejection = BitcodeRejection;
    
    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> Result<Self, Self::Rejection>
    {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(BitcodeRejection::Bytes)?;

        let structure = deserialize(&bytes).map_err(BitcodeRejection::Decode)?;
        
        Ok(Bitcode(structure))
    }
}

impl<T: Serialize> IntoResponse for Bitcode<T> {
    fn into_response(self) -> axum::response::Response {
        let bytes = serialize(&self.0).unwrap();

        (StatusCode::OK, bytes).into_response()
    }
}