use axum::{extract::rejection::{BytesRejection, FailedToBufferBody}, http::StatusCode, response::IntoResponse};
use bitcode::Error;

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