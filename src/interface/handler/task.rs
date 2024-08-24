use std::sync::Arc;

use crate::{
	auth::utils::auth::{auth_request, JWTAuthMiddleware},
	domain::{
		error::{Error, Result},
		task::{TaskModel, TaskService},
		sub::chat::ChatMsgService,
	},
	infra::types::FilterOptions,
	interface::dto::{
		task::req::{CreateTaskReq, TaskFilterOptions, UpdateTaskReq},
		sub::chat::req::{CreateMsgReq, UpdateMsgReq},
	},
	AppState,
};
use axum::{
	extract::{Path, Query, State},
	http::StatusCode,
	middleware,
	response::IntoResponse,
	routing::{get, post},
	Extension, Json, Router,
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
	let start_date = opts
		.start_date
		.map(|d| d.to_string()).unwrap_or_default();
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
		Ok(res) => Ok(Json(res)),
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
) -> Result<impl IntoResponse> {
	match TaskService::delete_task(&app_state.mongodb.db, &id)
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
