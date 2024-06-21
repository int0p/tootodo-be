use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Json, Router, middleware};

use crate::{
    auth::utils::auth::JWTAuthMiddleware,
    domain::{
        error::{Error, Result},
        habit::HabitService,
    },
    infra::types::FilterOptions,
    interface::dto::habit::req::{CreateHabitReq, HabitFilterOptions, UpdateHabitReq},
    AppState,
};
use crate::auth::utils::auth::auth_request;

pub fn habit_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/habits/", post(create_habit_handler))
        .route("/api/habits", get(habit_list_handler))
        .route(
            "/api/habits/:id",
            get(get_habit_handler)
                .patch(update_habit_handler)
                .delete(delete_habit_handler),
        )
        // .route("/api/habits/:id/records/", post(add_habit_record_handler))
        // .route("/api/habits/:id/records", get(fetch_habit_records_handler))
        // .route(
        //     "/api/habits/:id/records/:record_id",
        //     get(get_habit_record_handler)
        //         .delete(remove_habit_record_handler)
        //         .patch(update_habit_record_handler),
        // )
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_request,
        ))
        .with_state(app_state)
}

pub async fn habit_list_handler(
    opts: Option<Query<HabitFilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(25) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    // 날짜 입력 없으면 모든 habits 가져옴.
	let start_month = opts
    .start_month
    .map(|d| d.to_string()).unwrap_or_default();
    let end_month = opts.end_month.map(|d| d.to_string()).unwrap_or_default();


    match HabitService::fetch_habits(&app_state.mongodb.db, limit, page,&start_month,&end_month, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn create_habit_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateHabitReq>,
) -> Result<impl IntoResponse> {
    match HabitService::create_habit(&app_state.mongodb.db, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn get_habit_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match HabitService::get_habit(&app_state.mongodb.db, &id, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn update_habit_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<UpdateHabitReq>,
) -> Result<impl IntoResponse> {
    match HabitService::update_habit(&app_state.mongodb.db, &id, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn delete_habit_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    match HabitService::delete_habit(&app_state.mongodb.db, &id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}
