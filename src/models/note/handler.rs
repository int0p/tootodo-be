use std::sync::Arc;

use axum::{
    extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Extension, Json
};

use super::{
    error::{Error,Result},
    model::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
};

use crate::{auth::utils::auth::JWTAuthMiddleware, AppState};

pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match app_state
        .mongodb.note
        .fetch_notes(limit, page,&jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_note_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse> {
    match app_state.mongodb.note.create_note(&body,&jwtauth.user.id).await.map_err(Error::from) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_note_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match app_state.mongodb.note.get_note(&id,&jwtauth.user.id).await.map_err(Error::from) {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn edit_note_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse> {
    match app_state
        .mongodb.note
        .edit_note(&id, &body,&jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_note_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    match app_state.mongodb.note.delete_note(&id).await.map_err(Error::from) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e.into()),
    }
}