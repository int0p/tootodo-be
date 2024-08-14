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
        error::{Error, Result},
        sub::{chat::ChatMsgService, note_block::BlockService, note_propV::PropValueService},
        note::{NoteModel, NoteService},
    },
    infra::types::FilterOptions,
    interface::dto::{
        sub::{
            chat::req::{CreateMsgReq, UpdateMsgReq},
            note_block::req::{CreateBlockReq, UpdateBlockReq},
            note_propV::req::{CreatePropValueReq, UpdatePropValueReq},
        },
        note::req::{CreateNoteReq, UpdateNoteReq},
    },
    AppState,
};

pub fn note_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/notes/", post(create_note_handler))
        .route("/api/notes", get(note_list_handler))
        .route(
            "/api/notes/:id",
            get(get_note_handler)
                .patch(update_note_handler)
                .delete(delete_note_handler),
        )
        .route("/api/notes/:note_id/chat/", post(add_note_msg_handler))
        .route("/api/notes/:note_id/chat", get(fetch_note_msgs_handler))
        .route(
            "/api/notes/:note_id/chat/:msg_id",
            get(get_note_msg_handler)
                .delete(remove_note_msg_handler)
                .put(update_note_msg_handler),
        )
        .route("/api/notes/:note_id/blocks/", post(add_block_handler))
        .route("/api/notes/:note_id/blocks", get(fetch_blocks_handler))
        .route(
            "/api/notes/:note_id/blocks/:block_id",
            get(get_block_handler)
                .delete(remove_block_handler)
                .put(update_block_handler),
        )
        .route(
            "/api/notes/:note_id/prop_values/",
            post(add_note_propV_handler),
        )
        .route(
            "/api/notes/:note_id/prop_values",
            get(fetch_note_propVs_handler),
        )
        .route(
            "/api/notes/:note_id/prop_values/:prop_id",
            get(get_note_propV_handler)
                .delete(remove_note_propV_handler)
                .put(update_note_propV_handler),
        )
        .with_state(app_state)
}

// Task Handlers
pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match NoteService::fetch_notes(&app_state.mongodb.db, limit, page, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn create_note_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(mut body): Json<CreateNoteReq>,
) -> Result<impl IntoResponse> {
    match NoteService::create_note(&app_state.mongodb.db, &mut body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn get_note_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match NoteService::get_note(&app_state.mongodb.db, &id, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn update_note_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<UpdateNoteReq>,
) -> Result<impl IntoResponse> {
    match NoteService::update_note(&app_state.mongodb.db, &id, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn delete_note_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    match NoteService::delete_note(&app_state.mongodb.db, &id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

// Chat Handlers for Task
pub async fn get_note_msg_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id, msg_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match ChatMsgService::<NoteModel>::get_msg(&app_state.mongodb.db, &note_id, &msg_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn add_note_msg_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id,)): Path<(String,)>,
    Json(new_msg): Json<CreateMsgReq>,
) -> Result<impl IntoResponse> {
    match ChatMsgService::<NoteModel>::add_msg(&app_state.mongodb.db, &note_id, &new_msg)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn remove_note_msg_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id, msg_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match ChatMsgService::<NoteModel>::remove_msg(&app_state.mongodb.db, &note_id, &msg_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn update_note_msg_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id, msg_id)): Path<(String, String)>,
    Json(update_req): Json<UpdateMsgReq>,
) -> Result<impl IntoResponse> {
    match ChatMsgService::<NoteModel>::update_msg(
        &app_state.mongodb.db,
        &note_id,
        &msg_id,
        &update_req,
    )
    .await
    .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn fetch_note_msgs_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Path((note_id,)): Path<(String,)>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match ChatMsgService::<NoteModel>::fetch_msgs(&app_state.mongodb.db, &note_id, limit, page)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

// Task Block Handlers
pub async fn get_block_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id, block_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match BlockService::get_block(&app_state.mongodb.db, &note_id, &block_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn add_block_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id,)): Path<(String,)>,
    Json(new_block): Json<CreateBlockReq>,
) -> Result<impl IntoResponse> {
    match BlockService::add_block(&app_state.mongodb.db, &note_id, &new_block)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn remove_block_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id, block_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match BlockService::remove_block(&app_state.mongodb.db, &note_id, &block_id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

pub async fn update_block_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id, block_id)): Path<(String, String)>,
    Json(update_req): Json<UpdateBlockReq>,
) -> Result<impl IntoResponse> {
    match BlockService::update_block(&app_state.mongodb.db, &note_id, &block_id, &update_req)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn fetch_blocks_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id,)): Path<(String,)>,
) -> Result<impl IntoResponse> {
    match BlockService::fetch_blocks(&app_state.mongodb.db, &note_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

// Task Property Value Handlers
pub async fn get_note_propV_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id, prop_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match PropValueService::get_propV(&app_state.mongodb.db, &note_id, &prop_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn add_note_propV_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id,)): Path<(String,)>,
    Json(new_propV): Json<CreatePropValueReq>,
) -> Result<impl IntoResponse> {
    match PropValueService::add_propV(&app_state.mongodb.db, &note_id, new_propV)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn remove_note_propV_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id, prop_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match PropValueService::remove_propV(&app_state.mongodb.db, &note_id, &prop_id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

pub async fn update_note_propV_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id, prop_id)): Path<(String, String)>,
    Json(update_req): Json<UpdatePropValueReq>,
) -> Result<impl IntoResponse> {
    match PropValueService::update_propV(&app_state.mongodb.db, &note_id, &prop_id, &update_req)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn fetch_note_propVs_handler(
    State(app_state): State<Arc<AppState>>,
    Path((note_id,)): Path<(String,)>,
) -> Result<impl IntoResponse> {
    match PropValueService::fetch_propVs(&app_state.mongodb.db, &note_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}
