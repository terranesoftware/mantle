use axum::{http::StatusCode, response::IntoResponse};

pub enum MantleError {
    Sqlx(sqlx::Error)
}

impl From<sqlx::Error> for MantleError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl IntoResponse for MantleError {
    fn into_response(self) -> axum::response::Response {
        match self {
            // Log these internally later
            MantleError::Sqlx(sqlxe) => match sqlxe {
                sqlx::Error::AnyDriverError(_) |
                sqlx::Error::BeginFailed |
                sqlx::Error::ColumnDecode { .. } |
                sqlx::Error::ColumnIndexOutOfBounds { .. } |
                sqlx::Error::ColumnNotFound(_) |
                sqlx::Error::Configuration(_) |
                sqlx::Error::Database(_) |
                sqlx::Error::Decode(_) |
                sqlx::Error::Encode(_) |
                sqlx::Error::InvalidArgument(_) |
                sqlx::Error::InvalidSavePointStatement |
                sqlx::Error::Migrate(_) |
                sqlx::Error::Protocol(_) |
                sqlx::Error::TypeNotFound { .. } |
                sqlx::Error::WorkerCrashed => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND.into_response(),
                sqlx::Error::Io(_) |
                sqlx::Error::PoolClosed |
                sqlx::Error::PoolTimedOut |
                sqlx::Error::Tls(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}