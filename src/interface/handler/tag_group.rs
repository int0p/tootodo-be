use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use uuid::Uuid;

use crate::{
    auth::utils::auth::auth_request, domain::tag_relation::TagRelationService,
    interface::dto::relation::CreateTagRelationReq,
};
use crate::{
    auth::utils::auth::JWTAuthMiddleware,
    domain::{
        error::{Error, Result},
        tag_group::TagGroupService,
    },
    interface::dto::tag_group::req::{CreateTagGroupReq, UpdateTagGroupReq},
    AppState,
};

pub fn tag_group_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/tag_groups/", post(create_tag_group_handler))
        .route(
            "/api/tag_groups_all",
            get(tag_group_list_w_tag_handler).post(add_tag_to_group),
        )
        .route("/api/tag_groups", get(tag_group_list_handler))
        .route(
            "/api/tag_groups/:id",
            get(get_tag_group_handler)
                .patch(update_tag_group_handler)
                .delete(delete_tag_group_handler),
        )
        .route(
            "/api/tag_groups_all/:id",
            get(get_tag_group_w_tag_handler),
        )
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_request,
        ))
        .with_state(app_state)
}

pub async fn tag_group_list_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match TagGroupService::fetch_groups(&app_state.db, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn tag_group_list_w_tag_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match TagRelationService::fetch_groups_with_tags(&app_state.db, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn create_tag_group_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateTagGroupReq>,
) -> Result<impl IntoResponse> {
    match TagGroupService::create_group(&app_state.db, &jwtauth.user.id, body)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn get_tag_group_handler(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match TagGroupService::get_group(&app_state.db, &jwtauth.user.id, id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn get_tag_group_w_tag_handler(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match TagRelationService::get_group_with_tags(&app_state.db, &jwtauth.user.id, id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn update_tag_group_handler(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<UpdateTagGroupReq>,
) -> Result<impl IntoResponse> {
    match TagGroupService::update_group(&app_state.db, &jwtauth.user.id, id, body)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn add_tag_to_group(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateTagRelationReq>,
) -> Result<impl IntoResponse> {
    match TagRelationService::add_tag_to_group(&app_state.db, &jwtauth.user.id, body)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn delete_tag_group_handler(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    match TagGroupService::delete_group(&app_state.db, id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => {
            match TagRelationService::delete_relation(&app_state.db, id)
                .await
                .map_err(Error::from)
            {
                Ok(_) => Ok(StatusCode::NO_CONTENT),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}
