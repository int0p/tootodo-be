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
        tag::TagService,
    },
    interface::dto::tag::req::{CreateTagReq, UpdateTagReq},
    AppState,
};

pub fn tag_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/tags/", post(create_tag_handler))
        .route(
            "/api/tags_all",
            get(tag_group_list_w_tag_handler).post(assign_group),
        )
        .route("/api/tags", get(tag_list_handler))
        .route(
            "/api/tags/:id",
            get(get_tag_handler)
                .patch(update_tag_handler)
                .delete(delete_tag_handler),
        )
        .route("/api/tags_all/:id", get(get_tag_w_group_handler))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_request,
        ))
        .with_state(app_state)
}

pub async fn tag_list_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match TagService::fetch_tags(&app_state.db, &jwtauth.user.id)
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
    match TagRelationService::fetch_tags_with_groups(&app_state.db, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn create_tag_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateTagReq>,
) -> Result<impl IntoResponse> {
    match TagService::create_tag(&app_state.db, &jwtauth.user.id, body)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn get_tag_handler(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match TagService::get_tag(&app_state.db, &jwtauth.user.id, id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn get_tag_w_group_handler(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match TagRelationService::get_tag_with_groups(&app_state.db, &jwtauth.user.id, id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn update_tag_handler(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<UpdateTagReq>,
) -> Result<impl IntoResponse> {
    match TagService::update_tag(&app_state.db, &jwtauth.user.id, id, body)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn assign_group(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateTagRelationReq>,
) -> Result<impl IntoResponse> {
    match TagRelationService::assign_group(&app_state.db, &jwtauth.user.id, body)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn delete_tag_handler(
    Path(id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    match TagService::delete_tag(&app_state.db, id)
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
        }
        Err(e) => Err(e),
    }
}
