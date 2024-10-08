use std::sync::Arc;

use super::{
    error::{Error, Result},
    model::{LoginUserSchema, RegisterUserSchema, User},
    utils::{
        auth::{
            auth_first, filter_user_record, get_access_token_w_refresh, get_refresh_token_details,
            set_auth_cookies_header, AuthCookiesInfo, JWTAuthMiddleware,
        },
        google_oauth::{get_google_user, request_token, QueryCode},
        token,
    },
};
use crate::{infra::db, AppState};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::CookieJar;
use rand_core::OsRng;
use serde_json::json;
use tracing::info;

#[utoipa::path(
    get,
    path = "/api/healthchecker",
    tag = "Health Checker Endpoint",
    responses(
        (status = 200, description= "Authenticated User"),
    ),
    security(
        ("token" = [])
    )
)]
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Rust and Axum Framework: JWT Access and Refresh Tokens";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "Register Account Endpoint",
    request_body(content = RegisterUserSchema
        , description = "Credentials to create account"
        , example = json!({"email": "johndoe@example.com","name": "John Doe","password": "password123","passwordConfirm": "password123"})),
    responses(
        (status=201, description= "Account created successfully", body= RegisterUserSchema ),
        (status=400, description= "Validation Errors", body= ErrorResponse),
        (status=409, description= "User with email already exists", body= ErrorResponse),
        (status=500, description= "Internal Server Error", body= ErrorResponse ),
    )
)]
pub async fn register_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse> {
    info!("start register user");
    // email로 user검색
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 )")
            .bind(body.email.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| Error::DB(db::error::Error::Sqlx(e)))?;
    if let Some(exists) = user_exists {
        if exists {
            return Err(Error::UserAlreadyExists);
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(Error::CannotHashPassword)
        .map(|hash| hash.to_string())?;

    // user 등록
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
        body.name.to_string(),
        body.email.to_string().to_ascii_lowercase(),
        hashed_password,
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| Error::DB(db::error::Error::Sqlx(e)))?;

    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": filter_user_record(&user)
    })});

    Ok(Json(user_response))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Login Endpoint",
    request_body(content = LoginUserSchema
        , description = "Credentials to log in to your account"
        , example = json!({"email": "johndoe@example.com","password": "password123"})),
    responses(
        (status=200, description= "Login successfull", body= LoginUserResponse ),
        (status=400, description= "Validation Errors", body= ErrorResponse ),
        (status=500, description= "Internal Server Error", body= ErrorResponse ),
    )
)]
pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        body.email.to_ascii_lowercase()
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| Error::DB(db::error::Error::Sqlx(e)))?
    .ok_or_else(|| Error::InvalidLoginInfo)?;

    if user.provider != "local" {
        return Err(Error::WrongUserProvider);
    }

    let is_valid = match user.password.as_ref() {
        Some(password) => match PasswordHash::new(password) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(body.password.as_bytes(), &parsed_hash)
                .is_ok(),
            Err(_) => false,
        },
        None => false, // provider가 google일 땐 pw가 없고, local일 땐 반드시 존재.
    };
    if !is_valid {
        return Err(Error::InvalidLoginInfo);
    }

    let response = auth_first(user, &data).await?;

    Ok(response)
}

pub async fn refresh_access_token_handler(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    tracing::info!("CookieJar: {:?}", &cookie_jar);
    let refresh_token_details = get_refresh_token_details(&cookie_jar, &data).await?;
    tracing::info!(
        "Refresh Handler -> Refresh token: {:?}",
        &refresh_token_details
    );
    let access_token = get_access_token_w_refresh(&refresh_token_details, &data).await?;

    let mut response =
        Response::new(json!({"status": "success", "access_token": &access_token}).to_string());

    let refresh_token = refresh_token_details
        .token
        .as_ref()
        .ok_or(Error::EmptyToken)?;

    let cookies_info = AuthCookiesInfo {
        refresh_token,
        access_token: &access_token,
        login: true,
    };

    let headers = set_auth_cookies_header(&data, cookies_info)?;
    response.headers_mut().extend(headers);
    Ok(response)
}

#[utoipa::path(
    delete,
    path = "/api/auth/logout",
    tag = "Logout Endpoint",
    responses(
        (status=200, description= "Logout successfull" ),
        (status=400, description= "Validation Errors", body= ErrorResponse ),
        (status=401, description= "Unauthorize Error", body= ErrorResponse),
        (status=500, description= "Internal Server Error", body= ErrorResponse ),
    ),
    security(
       ("token" = [])
   )
)]

pub async fn logout_handler(
    cookie_jar: CookieJar,
    Extension(_auth_guard): Extension<JWTAuthMiddleware>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| Error::InvalidToken)?;

    let _refresh_token_details =
        match token::verify_jwt_token(data.env.refresh_token_public_key.to_owned(), &refresh_token)
        {
            Ok(token_details) => token_details,
            Err(e) => {
                return Err(Error::TokenDetails(e));
            }
        };

    let cookies_info = AuthCookiesInfo {
        refresh_token: &"".to_string(),
        access_token: &"".to_string(),
        login: false,
    };

    let headers = set_auth_cookies_header(&data, cookies_info)?;
    let mut response = Response::new(json!({"status": "success"}).to_string());
    response.headers_mut().extend(headers);
    Ok(response)
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "Get Authenticated User Endpoint",
    responses(
        (status = 200, description= "Authenticated User", body = UserResponse),
        (status= 500, description= "Internal Server Error", body = ErrorResponse )
    ),
    security(
       ("token" = [])
   )
)]

pub async fn get_me_handler(
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> axum::response::Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filter_user_record(&jwtauth.user)
        })
    });

    Ok(Json(json_response))
}

pub async fn google_oauth_handler(
    query: Query<QueryCode>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    info!("start google login");
    let code = &query.code;
    let state = &query.state;

    if code.is_empty() {
        return Err(Error::NoAuthCode);
    }

    let token_response = request_token(code.as_str(), &data).await;

    if let Err(e) = token_response {
        return Err(Error::TokenResponse(format!("{:?}", e)));
    }

    let token_response = token_response.unwrap();
    let google_user = get_google_user(&token_response.access_token, &token_response.id_token).await;
    if let Err(e) = google_user {
        return Err(Error::UserResponse(format!("{:?}", e)));
    }

    let google_user = google_user.unwrap();

    // find user in db
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1 ",
        google_user.email.to_ascii_lowercase()
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| Error::DB(db::error::Error::Sqlx(e)))?;

    // insert user if user not exists in db
    let user = match user {
        Some(user) => {
            if user.provider != "Google" {
                return Err(Error::WrongUserProvider);
            }else{
                user
            }
        }
        None => {
            sqlx::query_as!(
                User,
                "INSERT INTO users (email, name, provider, verified, photo) VALUES ($1, $2, 'Google', $3, $4) RETURNING *",
                google_user.email.to_ascii_lowercase(),
                google_user.name,
                google_user.verified_email,
                google_user.picture
            )
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                Error::DB(db::error::Error::Sqlx(e))
            })?
        }
    };

    let mut response = auth_first(user, &data).await?;
    let mut headers = HeaderMap::new();

    //redirect
    let frontend_origin = data.env.client_origin.to_owned();
    headers.append(
        header::LOCATION,
        format!("{}{}", frontend_origin, state)
            .to_string()
            .parse()
            .unwrap(),
    );

    response.headers_mut().extend(headers);
    Ok(response)
}
