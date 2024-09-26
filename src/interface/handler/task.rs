use std::{
    collections::{hash_map, HashMap},
    sync::Arc,
};

use crate::{
    auth::utils::auth::{auth_request, JWTAuthMiddleware},
    domain::{
        error::{Error, Result},
        sub::chat::ChatMsgService,
        task::{TaskModel, TaskService},
    },
    infra::types::{FilterOptions, TaskTreeItem},
    interface::dto::{
        sub::chat::req::{CreateMsgReq, UpdateMsgReq},
        task::{
            req::{CreateTaskReq, DeleteTaskOptionReq, TaskFilterOptions, UpdateTaskReq},
            res::{TaskListRes, TaskListTreeRes, TaskRes},
        },
    },
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post,delete,patch},
    Extension, Json, Router,
};
use mongodb::Database;
use uuid::Uuid;

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
        .route("/api/tasks/:task_id/chat", get(fetch_msgs_handler))
        .route(
            "/api/tasks/:task_id/chat/:msg_id",
            get(get_task_msg_handler)
                .delete(remove_task_msg_handler)
                .patch(update_task_msg_handler),
        )
        // .route(
        //     "/tasks/:task_id/chat/:msg_id/add_chat",
        //     post(add_chat_to_task_msg_handler),
        // )
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_request,
        ))
        .with_state(app_state)
}

pub async fn task_list_handler(
    opts: Option<Query<TaskFilterOptions>>,
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(100) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    // 날짜 입력 없으면 모든 tasks 가져옴.
    let start_date = opts.start_date.map(|d| d.to_string()).unwrap_or_default();
    let end_date = opts.end_date.map(|d| d.to_string()).unwrap_or_default();

    match TaskService::fetch_tasks(
        &app_state.mongodb.db,
        limit,
        page,
        &start_date,
        &end_date,
        &jwtauth.user.id,
    )
    .await
    .map_err(Error::from)
    {
        Ok(res) => {
            let task_tree =
                build_task_tree(&app_state.mongodb.db, &jwtauth.user.id, res.tasks).await?;
            Ok(Json(TaskListTreeRes {
                status: "success",
                results: task_tree.len(),
                tasks: task_tree,
            }))
        }
        Err(e) => Err(e),
    }
}

pub async fn create_task_handler(
    State(app_state): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateTaskReq>,
) -> Result<impl IntoResponse> {
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
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Json(option): Json<DeleteTaskOptionReq>,
) -> Result<impl IntoResponse> {
    
    match TaskService::delete_task(&app_state.mongodb.db, &id,option, &jwtauth.user.id)
        .await
        .map_err(Error::from)
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

// Chat Handlers for Event
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

pub async fn fetch_msgs_handler(
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

// TODO: add_chat_to_msg함수 구현된다면..
// pub async fn add_chat_to_task_msg_handler(
//     State(app_state): State<Arc<AppState>>,
//     Path((task_id, msg_id)): Path<(String, String)>,
// ) -> Result<impl IntoResponse> {
//     // Call the add_chat_to_msg function from the repository
//     let messages =
//         ChatMsgService::<EventModel>::add_chat_to_msg(&app_state.mongodb.db, &task_id, &msg_id)
//             .await
//             .unwrap();
//     Json(messages)
// }

async fn build_task_tree(
    db: &Database,
    user: &Uuid,
    tasks: Vec<TaskRes>,
) -> Result<Vec<TaskTreeItem>> {
    if tasks.is_empty() {
        return Ok(vec![]);
    }

    let mut task_item_map: HashMap<String, TaskTreeItem> = HashMap::new();
    for task in tasks {
        /*
          * tasks를 fetch할 때 sort옵션을 parent_id:1로 두어 parent_id가 없는 task가 먼저 나오도록 함
        parent_id가 없는 경우 HashMap에 추가
        parent_id가 있는 경우, map에서 parent_id를 찾아서 parent의 subtasks에 추가
        parent_id가 있는데, 해당 parent가 아직 map에 없다면? -> db에서 task검색하여 추가(fetch를 날짜에 대해 진행했으므로 입력받은 tasks에는 parent가 없을 수 있음)
         */
        if let Some(parent_id) = task.parent_id.clone().filter(|id| !id.is_empty()) {            
            tracing::info!("child task: {:?}, ", &task.start_date);
            let parent_task_item = match task_item_map.entry(parent_id.clone()) {
                hash_map::Entry::Occupied(entry) => entry.into_mut(),
                hash_map::Entry::Vacant(entry) => {
                    match TaskService::get_task(&db, &parent_id, user)
                        .await
                        .map_err(Error::from)
                    {
                        Ok(res) => entry.insert(TaskTreeItem {
                            task: res.data.task,
                            subtasks: Vec::new(),
                        }),
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            };
            parent_task_item.subtasks.push(TaskTreeItem {
                task,
                subtasks: Vec::new(),
            });
        } else {
            tracing::info!("task: {:?}, ", &task.start_date);
            task_item_map.insert(
                task.id.clone(),
                TaskTreeItem {
                    task: task.clone(),
                    subtasks: Vec::new(),
                },
            );
        };
    }

	 //  정렬된 결과 반환
    let mut task_tree: Vec<TaskTreeItem> = task_item_map.into_values().collect();
    task_tree.sort_by(|a, b| {
        a.task.start_date.cmp(&b.task.start_date)
            .then_with(|| b.task.end_date.cmp(&a.task.end_date))
    });
	 
    Ok(task_tree)
}
