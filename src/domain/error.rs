use crate::{error::ErrorResponse, infra::db};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use derive_more::From;
use utoipa::ToSchema;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, ToSchema, From)]
pub enum Error {
    #[from]
    DB(db::error::Error),

    #[from]
    MongoDuplicateError(mongodb::error::Error),
    NotFoundError(String),

    WrongUserAccess,

    TypedError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            Error::DB(e) => {
                return e.into_response();
            }
            Error::MongoDuplicateError(_) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "doc already exists".to_string(),
                },
            ),
            Error::NotFoundError(id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("ID: {} not found", id),
                },
            ),
            Error::WrongUserAccess => (
                StatusCode::FORBIDDEN,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "You do not have access to this note".to_string(),
                },
            ),
            Error::TypedError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("Wrong Input Type:{}", e),
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
