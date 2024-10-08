use crate::error::ErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use derive_more::From;
use uuid::Uuid;

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, From)]
pub enum Error {
    // postgresql
    FailToCreatePool(String),
    MigrationError(String),
    #[from]
    Sqlx(sqlx::Error),
    Fetch(sqlx::Error),
    EntityNotFound {
		entity: &'static str,
		id: Uuid,
	},
	ListLimitOverMax {
		max: i64,
		actual: i64,
	},

    // mongodb
    #[from]
    MongoError(mongodb::error::Error),
    #[from]
    MongoErrorKind(mongodb::error::ErrorKind),
    MongoQueryError(mongodb::error::Error),
    #[from]
    MongoGetOidError(mongodb::bson::oid::Error),
    #[from]
    MongoSerializeBsonError(mongodb::bson::ser::Error),
    #[from]
    MongoDataError(mongodb::bson::document::ValueAccessError),
    #[from]
    MongoDeserializeBsonError(mongodb::bson::de::Error),

}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            Error::EntityNotFound { entity, id } => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("{} with id: {} not found", entity, id),
                },
            ),
            Error::ListLimitOverMax { max, actual }=>(
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("List limit over max: max:{}, actual:{}", max, actual),
                },
            ),
            Error::MongoGetOidError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("MongoDB error: {}", e),
                },
            ),
            Error::Sqlx(e) => (
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
            Error::Fetch(e) => (
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
            Error::MongoDeserializeBsonError(e) => (
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
