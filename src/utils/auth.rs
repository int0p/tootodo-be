use std::sync::Arc;

use serde_json::json;

use crate::{error::Error, model::User, response::FilteredUser, AppState};

use super::token::{self, TokenDetails};

use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderMap, Request, Response},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use redis::AsyncCommands;

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

pub fn generate_token(
    user_id: uuid::Uuid,
    max_age: i64,
    private_key: String,
) -> Result<TokenDetails, Error> {
    token::generate_jwt_token(user_id, max_age, private_key)
        .map_err(|e| Error::GenerateTokenError(e))
}

pub async fn save_token_data_to_redis(
    data: &Arc<AppState>,
    token_details: &TokenDetails,
    max_age: i64,
) -> Result<(), Error> {
    let mut redis_client = data
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| Error::RedisError(e))?;
    redis_client
        .set_ex(
            token_details.token_uuid.to_string(),
            token_details.user_id.to_string(),
            (max_age * 60) as u64,
        )
        .await
        .map_err(|e| Error::RedisError(e))?;
    Ok(())
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

    save_token_data_to_redis(&data, &access_token_details, data.env.access_token_max_age).await?;
    save_token_data_to_redis(
        &data,
        &refresh_token_details,
        data.env.refresh_token_max_age,
    )
    .await?;

    /*
    HttpOnly 플래그: 이 플래그를 사용하면 JavaScript를 통한 쿠키의 접근을 차단할 수 있습니다. 따라서 XSS 공격으로부터 토큰을 보호할 수 있습니다. refresh_token은 특히 HttpOnly 플래그를 사용하여 저장해야 합니다.

    Secure 플래그: 이 플래그는 쿠키가 오직 HTTPS를 통해서만 전송되도록 합니다. 이는 중간자 공격을 방지하는 데 도움이 됩니다.

    SameSite 플래그: 이 설정은 쿠키가 cross-site 요청에 대해 어떻게 동작해야 하는지를 브라우저에 알려줍니다. SameSite=Lax 또는 SameSite=Strict를 설정하여 CSRF 공격을 방지할 수 있습니다.
     */
    let access_cookie = Cookie::build((
        "access_token",
        access_token_details.token.clone().unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true).build();

    let refresh_cookie = Cookie::build((
        "refresh_token",
        refresh_token_details.token.unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(data.env.refresh_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true).build();

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(false).build();

    let headers = append_cookies_to_headers(vec![access_cookie, refresh_cookie, logged_in_cookie]);

    let mut response = Response::new(
        json!({"status": "success", "access_token": access_token_details.token.unwrap()})
            .to_string(),
    );
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

    let access_token_details =
        match token::verify_jwt_token(data.env.access_token_public_key.to_owned(), &access_token) {
            Ok(token_details) => token_details,
            Err(e) => {
                return Err(Error::VerifyTokenError(e));
            }
        };

    let access_token_uuid = uuid::Uuid::parse_str(&access_token_details.token_uuid.to_string())
        .map_err(|_| Error::InvalidToken)?;

    let mut redis_client = data
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| Error::RedisError(e))?;

    let redis_token_user_id = redis_client
        .get::<_, String>(access_token_uuid.clone().to_string())
        .await
        .map_err(|_| Error::InvalidToken)?;

    let user_id_uuid =
        uuid::Uuid::parse_str(&redis_token_user_id).map_err(|_| Error::InvalidToken)?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id_uuid)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| Error::FetchError(e))?;

    let user = user.ok_or_else(|| Error::NoUser)?;

    req.extensions_mut().insert(JWTAuthMiddleware {
        user,
        access_token_uuid,
    });
    Ok(next.run(req).await)
}

pub fn append_cookies_to_headers(cookies: Vec<Cookie<'static>>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for cookie in cookies {
        headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    }
    headers
}
