use crate::auth;
use crate::db;
use crate::model;
use axum::http;
use derive_more::From;
use serde::Serialize;
use utoipa::ToSchema;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize,ToSchema)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, From)]
pub enum Error {
	// -- Modules
	#[from]
	Auth(auth::error::Error),
    #[from]
    DB(db::error::Error),
    #[from]
    Model(model::error::Error),

	IOError(std::io::Error),
	HeaderError(http::header::InvalidHeaderValue),
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