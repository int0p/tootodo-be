use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use super::{
    controller::MemoBMC,
    schema::{CreateMemoSchema, FilterOptions, UpdateMemoSchema},
};

use crate::{
    auth::utils::auth::JWTAuthMiddleware,
    models::error::{Error, Result},
    AppState,
};

pub async fn memo_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match MemoBMC::fetch_memos(&app_state.mongodb.db, limit, page, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_memo_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateMemoSchema>,
) -> Result<impl IntoResponse> {
    match MemoBMC::create_memo(&app_state.mongodb.db, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_memo_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match MemoBMC::get_memo(&app_state.mongodb.db, &id, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn update_memo_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<UpdateMemoSchema>,
) -> Result<impl IntoResponse> {
    match MemoBMC::update_memo(&app_state.mongodb.db, &id, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_memo_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    match MemoBMC::delete_memo(&app_state.mongodb.db, &id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e.into()),
    }
}
