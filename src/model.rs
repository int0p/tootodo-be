use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub provider: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct RegisterUserSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        // length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Please confirm your password"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginUserSchema {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        // length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginUserResponse {
    pub access_token: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FilterUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUser {
    pub fn filter_user(user: &User) -> Self {
        FilterUser {
            id: user.id.to_string(),
            email: user.email.to_owned(),
            name: user.name.to_owned(),
            photo: user.photo.to_owned(),
            role: user.role.as_str().to_string(),
            verified: user.verified,
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    pub fn filter_users(users: &[User]) -> Vec<FilterUser> {
        users.iter().map(FilterUser::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserData {
    pub user: FilterUser,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}