use argon2::password_hash;
use utoipa::ToSchema;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{db, error::ErrorResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug,ToSchema)]
pub enum Error {
    DB(db::error::Error),
    //auth
    CannotHashPassword(password_hash::Error),
    UserAlreadyExists,
    InvalidLoginInfo,
    WrongUserProvider,
    GenerateTokenError(jsonwebtoken::errors::Error),
    RefreshTokenError,
    TokenDetailsError(jsonwebtoken::errors::Error),
    InvalidToken,
    NoUser,
    NoAuthCode,
    RetrieveTokenError(String),
    RetriveUserError,
    TokenResponseError(String),
    UserResponseError(String),
    NoAccessToken,
    VerifyTokenError(jsonwebtoken::errors::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            Error::DB(e) => {
                return e.into_response();            
            },
            Error::VerifyTokenError(e) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("Error verifying token: {}", e),
                }
            ),
            Error::NoAccessToken => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "No access token provided".to_string(),
                },
            ),
            
            Error::CannotHashPassword(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("Error while hashing password: {}", e),
                },
            ),
            Error::UserAlreadyExists => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "User already exists.".to_string(),
                },
            ),
            Error::InvalidLoginInfo => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid email or password.".to_string(),
                },
            ),
            Error::WrongUserProvider => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "User registered with a different provider".to_string(),
                },
            ),
            Error::GenerateTokenError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("error generating token: {}", e),
                },
            ),
            Error::RefreshTokenError => (
                StatusCode::FORBIDDEN,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "could not refresh access token".to_string(),
                },
            ),
            Error::TokenDetailsError(e) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    status: "error".to_string(),
                    message: format!("error getting token details: {}", e),
                },
            ),
            Error::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "Token is invalid or session has expired".to_string(),
                },
            ),           
            Error::NoUser => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "The user belonging to this token no longer exists".to_string(),
                },
            ),
            Error::NoAuthCode => (
                StatusCode::BAD_GATEWAY,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "Authorization code not provided!".to_string(),
                },
            ),            
            Error::RetrieveTokenError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("{:?}", e),
                },
            ),
            Error::RetriveUserError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: "An error occurred while trying to retrieve user information.".to_string(),
                },
            ),
            Error::TokenResponseError(e) => (
                StatusCode::BAD_GATEWAY,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("{:?}", e),
                },
            ),
            Error::UserResponseError(e) => (
                StatusCode::BAD_GATEWAY,
                ErrorResponse {
                    status: "fail".to_string(),
                    message: format!("{:?}", e),
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
