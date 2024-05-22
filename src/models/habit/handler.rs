use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};

use super::{
    controller::HabitBMC,
    schema::{CreateHabitSchema, FilterOptions, UpdateHabitSchema},
};

use crate::{
    auth::utils::auth::JWTAuthMiddleware,
    models::error::{Error, Result},
    AppState,
};

pub async fn habit_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match HabitBMC::fetch_habits(&app_state.mongodb.db, limit, page, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_habit_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateHabitSchema>,
) -> Result<impl IntoResponse> {
    match HabitBMC::create_habit(&app_state.mongodb.db, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_habit_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match HabitBMC::get_habit(&app_state.mongodb.db, &id, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn update_habit_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<UpdateHabitSchema>,
) -> Result<impl IntoResponse> {
    match HabitBMC::update_habit(&app_state.mongodb.db, &id, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_habit_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    match HabitBMC::delete_habit(&app_state.mongodb.db, &id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e.into()),
    }
}
