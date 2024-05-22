use crate::auth;
use crate::db;
use crate::models;
use axum::http;
use derive_more::From;
use serde::Serialize;
use utoipa::ToSchema;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, ToSchema)]
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
    Model(models::error::Error),

    ServerError(std::io::Error),
    HeaderError(http::header::InvalidHeaderValue),

    CtxCannotNewRootCtx,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
