use std::sync::Arc;

use serde_json::json;

use crate::{
    auth::{error::Error, model::User, response::FilteredUser},
    infra::db,
    AppState,
};

use super::token::{self, TokenDetails};

use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderMap, Request, Response},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
// use redis::AsyncCommands;

use axum_extra::extract::cookie::CookieJar;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddleware {
    pub user: User,
    pub access_token_uuid: uuid::Uuid,
}

pub fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        photo: user.photo.to_owned(),
        role: user.role.to_owned(),
        verified: user.verified,
        provider: user.provider.to_owned(),
        createdAt: user.created_at.unwrap(),
        updatedAt: user.updated_at.unwrap(),
    }
}

pub struct AuthCookiesInfo<'a> {
    pub refresh_token: &'a String,
    pub access_token: &'a String,
    pub login: bool,
}

pub fn generate_token(
    user_id: uuid::Uuid,
    max_age: i64,
    private_key: String,
) -> Result<TokenDetails, Error> {
    token::generate_jwt_token(user_id, max_age, private_key).map_err(Error::GenerateToken)
}

//사용자 인증을 위해 새로운 토큰을 생성하고, 이를 레디스에 저장한 후 쿠키를 생성하고 응답에 추가
pub async fn auth_first(user: User, data: &Arc<AppState>) -> Result<Response<String>, Error> {
    let access_token_details = generate_token(
        user.id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    )?;
    let refresh_token_details = generate_token(
        user.id,
        data.env.refresh_token_max_age,
        data.env.refresh_token_private_key.to_owned(),
    )?;

    let tokens = vec![
        refresh_token_details
            .token
            .as_ref()
            .ok_or(Error::EmptyToken)?,
        access_token_details
            .token
            .as_ref()
            .ok_or(Error::EmptyToken)?,
    ];

    let cookies_info = AuthCookiesInfo {
        refresh_token: tokens[0],
        access_token: tokens[1],
        login: true,
    };

    let headers = set_auth_cookies_header(data, cookies_info)?;

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response.headers_mut().extend(headers);

    Ok(response)
}

//요청에서 액세스 토큰을 추출하고 검증한 후, 사용자 정보를 요청에 추가하여 다음 처리 단계로 전달합니다.
pub async fn auth_request(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Error> {
    // tracing::info!("Auth Request -> CookieJar:{:?}", &cookie_jar);
    let access_token = cookie_jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });
    let access_token = access_token.ok_or_else(|| Error::NoAccessToken)?;
    // tracing::info!("Auth_Request -> Access token: {:?}", &access_token);

    let access_token_details =
        token::verify_jwt_token(data.env.access_token_public_key.to_owned(), &access_token)
            .map_err(|e| Error::VerifyToken(e))?;
        
    let access_token_uuid = uuid::Uuid::parse_str(&access_token_details.token_uuid.to_string())
        .map_err(|_| Error::InvalidToken)?;

    let user_id_uuid = uuid::Uuid::parse_str(&access_token_details.user_id.to_string())
        .map_err(|_| Error::InvalidToken)?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id_uuid)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| Error::DB(db::error::Error::Fetch(e)))?;

    let user = user.ok_or_else(|| Error::NoUser)?;

    req.extensions_mut().insert(JWTAuthMiddleware {
        user,
        access_token_uuid,
    });

    Ok(next.run(req).await)
}

pub fn set_auth_cookies_header(
    data: &Arc<AppState>,
    details: AuthCookiesInfo,
) -> Result<HeaderMap, Error> {
    /*
    HttpOnly 플래그: 이 플래그를 사용하면 JavaScript를 통한 쿠키의 접근을 차단할 수 있습니다. 따라서 XSS 공격으로부터 토큰을 보호할 수 있습니다. refresh_token은 특히 HttpOnly 플래그를 사용하여 저장해야 합니다.

    Secure 플래그: 이 플래그는 쿠키가 오직 HTTPS를 통해서만 전송되도록 합니다. 이는 중간자 공격을 방지하는 데 도움이 됩니다.

    SameSite 플래그: 이 설정은 쿠키가 cross-site 요청에 대해 어떻게 동작해야 하는지를 브라우저에 알려줍니다. SameSite=Lax 또는 SameSite=Strict를 설정하여 CSRF 공격을 방지할 수 있습니다.
     */
    let refresh_cookie = Cookie::build(("refresh_token", details.refresh_token))
        .path("/")
        .domain(&data.env.domain)
        .max_age(time::Duration::minutes(data.env.refresh_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let access_cookie = Cookie::build(("access_token", details.access_token))
        .path("/")
        .domain(&data.env.domain)
        .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let logged_in_cookie = Cookie::build(("logged_in", details.login.to_string()))
        .path("/")
        .domain(&data.env.domain)
        .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(false)
        .build();

    Ok(append_cookies_to_headers(vec![
        access_cookie,
        refresh_cookie,
        logged_in_cookie,
    ]))
}

pub async fn get_refresh_token_details(
    cookie_jar: &CookieJar,
    data: &Arc<AppState>,
) -> Result<TokenDetails, Error> {
    
    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or(Error::RefreshToken)?;

    // refresh_token 검증 -> detail 반환
    token::verify_jwt_token(data.env.refresh_token_public_key.to_owned(), &refresh_token)
        .map_err(|e| Error::VerifyToken(e))
}

pub async fn get_access_token_w_refresh(
    refresh_token_details: &TokenDetails,
    data: &Arc<AppState>,
) -> Result<String, Error> {
    let user_id_uuid = uuid::Uuid::parse_str(&refresh_token_details.user_id.to_string())
        .map_err(|_| Error::InvalidToken)?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id_uuid)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| Error::DB(db::error::Error::Fetch(e)))?;

    let user = user.ok_or_else(|| Error::NoUser)?;

    let access_token_details = generate_token(
        user.id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    )?;

    Ok(access_token_details.token.unwrap_or_default())
}

pub fn append_cookies_to_headers(cookies: Vec<Cookie>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for cookie in cookies {
        headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    }
    headers
}
