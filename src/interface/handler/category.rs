use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use std::sync::Arc;

use crate::{
    auth::utils::auth::JWTAuthMiddleware,
    domain::{
        category::CategoryService,
        error::{Error, Result},
        sub::property::PropertyService,
        note::NoteService,
    },
    infra::types::FilterOptions,
    interface::dto::{
        tag_group::req::{CreateCategoryReq, UpdateCategoryReq},
        sub::property::req::{CreatePropertyReq, UpdatePropertyReq},
    },
    AppState,
};

pub fn category_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/categories/", post(create_category_handler))
        .route("/api/categories", get(category_list_handler))
        .route(
            "/api/categories/:id",
            get(get_category_handler)
                .patch(update_category_handler)
                .delete(delete_category_handler),
        )
        .route(
            "/api/categories/:category_id/properties/",
            post(add_property_handler),
        )
        .route(
            "/api/categories/:category_id/properties",
            get(fetch_properties_handler),
        )
        .route(
            "/api/categories/:category_id/properties/:property_id",
            get(get_property_handler)
                .delete(remove_property_handler)
                .put(update_property_handler),
        )
        .with_state(app_state)
}

pub async fn category_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match CategoryService::fetch_categories(&app_state.mongodb.db, limit, page, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn create_category_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateCategoryReq>,
) -> Result<impl IntoResponse> {
    match CategoryService::create_category(&app_state.mongodb.db, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn get_category_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match CategoryService::get_category(&app_state.mongodb.db, &id, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn update_category_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<UpdateCategoryReq>,
) -> Result<impl IntoResponse> {
    match CategoryService::update_category(&app_state.mongodb.db, &id, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => {
            // Update related notes
            if let (Some(name), Some(color)) = (&body.name, &body.color) {
                NoteService::update_notes_for_category_change(
                    &app_state.mongodb.db,
                    &id,
                    name,
                    color,
                    &jwtauth.user.id,
                )
                .await
                .map_err(Error::from)?;
            }

            Ok(Json(res))
        }
        Err(e) => Err(e),
    }
}

pub async fn delete_category_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    match CategoryService::delete_category(&app_state.mongodb.db, &id, None)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

// Property Handlers
pub async fn get_property_handler(
    State(app_state): State<Arc<AppState>>,
    Path((category_id, property_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match PropertyService::get_property(&app_state.mongodb.db, &category_id, &property_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn add_property_handler(
    State(app_state): State<Arc<AppState>>,
    Path((category_id,)): Path<(String,)>,
    Json(new_property): Json<CreatePropertyReq>,
) -> Result<impl IntoResponse> {
    match PropertyService::add_property(&app_state.mongodb.db, &category_id, &new_property)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn remove_property_handler(
    State(app_state): State<Arc<AppState>>,
    Path((category_id, property_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match PropertyService::remove_property(&app_state.mongodb.db, &category_id, &property_id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

pub async fn update_property_handler(
    State(app_state): State<Arc<AppState>>,
    Path((category_id, prop_id)): Path<(String, String)>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(update_req): Json<UpdatePropertyReq>,
) -> Result<impl IntoResponse> {
    match PropertyService::update_property(
        &app_state.mongodb.db,
        &category_id,
        &prop_id,
        &update_req,
    )
    .await
    .map_err(Error::from)
    {
        Ok(res) => {
            // Update related notes
            if let (Some(name), Some(prop_type)) = (&update_req.name, &update_req.prop_type) {
                NoteService::update_notes_for_property_change(
                    &app_state.mongodb.db,
                    &category_id,
                    &prop_id,
                    name,
                    prop_type,
                    &jwtauth.user.id,
                )
                .await
                .map_err(Error::from)?;
            }

            Ok(Json(res))
        }
        Err(e) => Err(e),
    }
}

pub async fn fetch_properties_handler(
    State(app_state): State<Arc<AppState>>,
    Path((category_id,)): Path<(String,)>,
) -> Result<impl IntoResponse> {
    match PropertyService::fetch_properties(&app_state.mongodb.db, &category_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}
