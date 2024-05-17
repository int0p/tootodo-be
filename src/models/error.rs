use crate::{db,error::ErrorResponse};
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use utoipa::ToSchema;
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug,ToSchema,From)]
pub enum Error {
	// Auth(auth::error::Error),
    #[from]
    DB(db::error::Error),

    #[from]
	MongoDuplicateError(mongodb::error::Error),
    NotFoundError(String),
	
	WrongUserAccess,
}


impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            Error::DB(e) => {
                return e.into_response();            
            },
            Error::MongoDuplicateError(_) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "Note with that title already exists".to_string(),
                },
            ),
            Error::NotFoundError(id) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("Note with ID: {} not found", id),
                },
            ),
            Error::WrongUserAccess => (
                StatusCode::FORBIDDEN,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "You do not have access to this note".to_string(),
                },
            ),
        };
        (status, Json(serde_json::to_value(error_response).unwrap())).into_response()
    }
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate