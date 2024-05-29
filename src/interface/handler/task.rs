use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use std::sync::Arc;

use crate::{
    auth::utils::auth::JWTAuthMiddleware,
    domain::{
        error::{Error, Result},
        sub::{chat::ChatMsgService, task_block::BlockService, task_propV::PropValueService},
        task::{TaskModel, TaskService},
    },
    interface::dto::{
        sub::{
            chat::req::{CreateMsgReq, UpdateMsgReq},
            task_block::req::{CreateBlockReq, UpdateBlockReq},
            task_propV::req::{CreatePropValueReq, UpdatePropValueReq},
        },
        task::req::{CreateTaskReq, UpdateTaskReq},
        FilterOptions,
    },
    AppState,
};

pub fn task_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/tasks/", post(create_task_handler))
        .route("/api/tasks", get(task_list_handler))
        .route(
            "/api/tasks/:id",
            get(get_task_handler)
                .patch(update_task_handler)
                .delete(delete_task_handler),
        )
        .route("/api/tasks/:task_id/chat/", post(add_task_msg_handler))
        .route("/api/tasks/:task_id/chat", get(fetch_task_msgs_handler))
        .route(
            "/api/tasks/:task_id/chat/:msg_id",
            get(get_task_msg_handler)
                .delete(remove_task_msg_handler)
                .put(update_task_msg_handler),
        )
        .route("/api/tasks/:task_id/blocks/", post(add_block_handler))
        .route("/api/tasks/:task_id/blocks", get(fetch_blocks_handler))
        .route(
            "/api/tasks/:task_id/blocks/:block_id",
            get(get_block_handler)
                .delete(remove_block_handler)
                .put(update_block_handler),
        )
        .route(
            "/api/tasks/:task_id/prop_values/",
            post(add_task_propV_handler),
        )
        .route(
            "/api/tasks/:task_id/prop_values",
            get(fetch_task_propVs_handler),
        )
        .route(
            "/api/tasks/:task_id/prop_values/:prop_id",
            get(get_task_propV_handler)
                .delete(remove_task_propV_handler)
                .put(update_task_propV_handler),
        )
        .with_state(app_state)
}

// Task Handlers
pub async fn task_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match TaskService::fetch_tasks(&app_state.mongodb.db, limit, page, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn create_task_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateTaskReq>,
) -> Result<impl IntoResponse> {
    // Property 정보 추가. --- front에서 해야하나?
    // let category_collection = db.collection::<CategoryModel>("categories");
    // let category = category_collection
    //     .find_one(doc! { "_id": task_result.category_id }, None)
    //     .await
    //     .expect("Failed to fetch category")
    //     .expect("Category not found");

    // let properties: Vec<PropValueModel> = category
    //     .props
    //     .iter()
    //     .map(|prop| PropValueModel {
    //         prop_id: prop.id,
    //         prop_name: prop.name.clone(),
    //         value: None,
    //         prop_type: prop.prop_type.clone(),
    //     })
    //     .collect();
    // TODO: body에 Property 정보 추가.
    match TaskService::create_task(&app_state.mongodb.db, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn get_task_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    match TaskService::get_task(&app_state.mongodb.db, &id, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn update_task_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<UpdateTaskReq>,
) -> Result<impl IntoResponse> {
    match TaskService::update_task(&app_state.mongodb.db, &id, &body, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn delete_task_handler(
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    match TaskService::delete_task(&app_state.mongodb.db, &id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

// Chat Handlers for Task
pub async fn get_task_msg_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id, msg_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match ChatMsgService::<TaskModel>::get_msg(&app_state.mongodb.db, &task_id, &msg_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn add_task_msg_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id,)): Path<(String,)>,
    Json(new_msg): Json<CreateMsgReq>,
) -> Result<impl IntoResponse> {
    match ChatMsgService::<TaskModel>::add_msg(&app_state.mongodb.db, &task_id, &new_msg)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn remove_task_msg_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id, msg_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match ChatMsgService::<TaskModel>::remove_msg(&app_state.mongodb.db, &task_id, &msg_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn update_task_msg_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id, msg_id)): Path<(String, String)>,
    Json(update_req): Json<UpdateMsgReq>,
) -> Result<impl IntoResponse> {
    match ChatMsgService::<TaskModel>::update_msg(
        &app_state.mongodb.db,
        &task_id,
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

pub async fn fetch_task_msgs_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Path((task_id,)): Path<(String,)>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match ChatMsgService::<TaskModel>::fetch_msgs(&app_state.mongodb.db, &task_id, limit, page)
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
    Path((task_id, block_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match BlockService::get_block(&app_state.mongodb.db, &task_id, &block_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn add_block_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id,)): Path<(String,)>,
    Json(new_block): Json<CreateBlockReq>,
) -> Result<impl IntoResponse> {
    match BlockService::add_block(&app_state.mongodb.db, &task_id, &new_block)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn remove_block_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id, block_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match BlockService::remove_block(&app_state.mongodb.db, &task_id, &block_id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

pub async fn update_block_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id, block_id)): Path<(String, String)>,
    Json(update_req): Json<UpdateBlockReq>,
) -> Result<impl IntoResponse> {
    match BlockService::update_block(&app_state.mongodb.db, &task_id, &block_id, &update_req)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn fetch_blocks_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Path((task_id,)): Path<(String,)>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match BlockService::fetch_blocks(&app_state.mongodb.db, &task_id, limit, page)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

// Task Property Value Handlers
pub async fn get_task_propV_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id, prop_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match PropValueService::get_propV(&app_state.mongodb.db, &task_id, &prop_id)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn add_task_propV_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id,)): Path<(String,)>,
    Json(new_propV): Json<CreatePropValueReq>,
) -> Result<impl IntoResponse> {
    match PropValueService::add_propV(&app_state.mongodb.db, &task_id, new_propV)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn remove_task_propV_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id, prop_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    match PropValueService::remove_propV(&app_state.mongodb.db, &task_id, &prop_id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

pub async fn update_task_propV_handler(
    State(app_state): State<Arc<AppState>>,
    Path((task_id, prop_id)): Path<(String, String)>,
    Json(update_req): Json<UpdatePropValueReq>,
) -> Result<impl IntoResponse> {
    match PropValueService::update_propV(&app_state.mongodb.db, &task_id, &prop_id, &update_req)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn fetch_task_propVs_handler(
    opts: Option<Query<FilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Path((task_id,)): Path<(String,)>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    match PropValueService::fetch_propVs(&app_state.mongodb.db, &task_id, limit, page)
        .await
        .map_err(Error::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}
