use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use crate::error::ErrorResponse;

#[derive(Debug)]
pub enum Error {
    MongoError(mongodb::error::Error),
    MongoErrorKind(mongodb::error::ErrorKind),
    MongoDuplicateError(mongodb::error::Error),
    MongoQueryError(mongodb::error::Error),
    MongoSerializeBsonError(mongodb::bson::ser::Error),
    MongoDataError(mongodb::bson::document::ValueAccessError),
    InvalidIDError(String),
    NotFoundError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            Error::MongoErrorKind(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("MongoDB error kind: {}", e),
                },
            ),
            Error::MongoDuplicateError(_) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "Note with that title already exists".to_string(),
                },
            ),
            Error::InvalidIDError(id) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("invalid ID: {}", id),
                },
            ),
            Error::NotFoundError(id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("Note with ID: {} not found", id),
                },
            ),
            Error::MongoError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("MongoDB error: {}", e),
                },
            ),
            Error::MongoQueryError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("MongoDB error: {}", e),
                },
            ),
            Error::MongoSerializeBsonError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("MongoDB error: {}", e),
                },
            ),
            Error::MongoDataError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("MongoDB error: {}", e),
                },
            ),
        };
        (status, Json(serde_json::to_value(error_response).unwrap())).into_response()
    }
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
