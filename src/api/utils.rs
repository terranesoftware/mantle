use axum::{body::Bytes, extract::{FromRequest, rejection::{BytesRejection, FailedToBufferBody}}, http::StatusCode, response::IntoResponse};
use bitcode::{Error, deserialize};
use serde::de::DeserializeOwned;

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

pub enum BitcodeRejection {
    Bytes(BytesRejection),
    Decode(Error)
}

impl IntoResponse for BitcodeRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            BitcodeRejection::Bytes(br) => match br {
                BytesRejection::FailedToBufferBody(ftbb) => match ftbb {
                    FailedToBufferBody::UnknownBodyError(ube) => ube.into_response(),
                    _ => unreachable!()
                }
                _ => (StatusCode::BAD_REQUEST, "Unknown error").into_response()
            }
            BitcodeRejection::Decode(dr) => (StatusCode::BAD_REQUEST, format!("{}", dr)).into_response()
        }
    }
}