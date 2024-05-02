use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use crate::error::ErrorResponse;
pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    // postgresql
    FailToCreatePool(String),
    MigrationError(String),
    DatabaseError(sqlx::Error),
    FetchError(sqlx::Error),

    // mongodb
    MongoError(mongodb::error::Error),
    MongoErrorKind(mongodb::error::ErrorKind),
    MongoQueryError(mongodb::error::Error),
    MongoSerializeBsonError(mongodb::bson::ser::Error),
    MongoDataError(mongodb::bson::document::ValueAccessError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            Error::DatabaseError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("Database error: {}", e),
                },
            ),
            Error::FailToCreatePool(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("🔥 Failed to connect to the database: {:?}", e),
                },
            ),
            Error::MigrationError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("Error executing migrations: {}", e),
                },
            ),            
            Error::FetchError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("Error fetching user from database:  {}", e),
                },
            ),

            Error::MongoErrorKind(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("MongoDB error kind: {}", e),
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
