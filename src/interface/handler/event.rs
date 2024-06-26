use std::sync::Arc;

use crate::{
	auth::utils::auth::{auth_request, JWTAuthMiddleware},
	domain::{
		error::{Error, Result},
		event::{EventModel, EventService},
		sub::chat::ChatMsgService,
	},
	infra::types::FilterOptions,
	interface::dto::{
		event::req::{CreateEventReq, EventFilterOptions, UpdateEventReq},
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
use chrono::Utc;

pub fn event_router(app_state: Arc<AppState>) -> Router {
	Router::new()
		.route("/api/events/", post(create_event_handler))
		.route("/api/events", get(event_list_handler))
		.route(
				"/api/events/:id",
				get(get_event_handler)
					.patch(update_event_handler)
					.delete(delete_event_handler),
		)
		.route("/api/events/:event_id/chat/", post(add_event_msg_handler))
		.route("/api/events/:event_id/chat", get(fetch_msgs_handler))
		.route(
				"/api/events/:event_id/chat/:msg_id",
				get(get_event_msg_handler)
					.delete(remove_event_msg_handler)
					.patch(update_event_msg_handler),
		)
		// .route(
		//     "/events/:event_id/chat/:msg_id/add_chat",
		//     post(add_chat_to_event_msg_handler),
		// )
		.route_layer(middleware::from_fn_with_state(
				app_state.clone(),
				auth_request,
		))
		.with_state(app_state)
}

pub async fn event_list_handler(
	opts: Option<Query<EventFilterOptions>>,
	State(app_state): State<Arc<AppState>>,
	Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
	let Query(opts) = opts.unwrap_or_default();

	let limit = opts.limit.unwrap_or(25) as i64;
	let page = opts.page.unwrap_or(1) as i64;
	
	// 날짜 입력 없으면 모든 events 가져옴.
	let start_date = opts
		.start_date
		.map(|d| d.to_string()).unwrap_or_default();
	let end_date = opts.end_date.map(|d| d.to_string()).unwrap_or_default();

	match EventService::fetch_events(
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

pub async fn create_event_handler(
	State(app_state): State<Arc<AppState>>,
	Extension(jwtauth): Extension<JWTAuthMiddleware>,
	Json(body): Json<CreateEventReq>,
) -> Result<impl IntoResponse> {
	match EventService::create_event(&app_state.mongodb.db, &body, &jwtauth.user.id)
		.await
		.map_err(Error::from)
	{
		Ok(res) => Ok(Json(res)),
		Err(e) => Err(e),
	}
}

pub async fn get_event_handler(
	Path(id): Path<String>,
	State(app_state): State<Arc<AppState>>,
	Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
	match EventService::get_event(&app_state.mongodb.db, &id, &jwtauth.user.id)
		.await
		.map_err(Error::from)
	{
		Ok(res) => Ok(Json(res)),
		Err(e) => Err(e),
	}
}

pub async fn update_event_handler(
	Path(id): Path<String>,
	State(app_state): State<Arc<AppState>>,
	Extension(jwtauth): Extension<JWTAuthMiddleware>,
	Json(body): Json<UpdateEventReq>,
) -> Result<impl IntoResponse> {
	match EventService::update_event(&app_state.mongodb.db, &id, &body, &jwtauth.user.id)
		.await
		.map_err(Error::from)
	{
		Ok(res) => Ok(Json(res)),
		Err(e) => Err(e),
	}
}

pub async fn delete_event_handler(
	Path(id): Path<String>,
	State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
	match EventService::delete_event(&app_state.mongodb.db, &id)
		.await
		.map_err(Error::from)
	{
		Ok(_) => Ok(StatusCode::NO_CONTENT),
		Err(e) => Err(e),
	}
}

// Chat Handlers for Event
pub async fn get_event_msg_handler(
	State(app_state): State<Arc<AppState>>,
	Path((event_id, msg_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
	match ChatMsgService::<EventModel>::get_msg(&app_state.mongodb.db, &event_id, &msg_id)
		.await
		.map_err(Error::from)
	{
		Ok(res) => Ok(Json(res)),
		Err(e) => Err(e),
	}
}

pub async fn add_event_msg_handler(
	State(app_state): State<Arc<AppState>>,
	Path((event_id,)): Path<(String,)>,
	Json(new_msg): Json<CreateMsgReq>,
) -> Result<impl IntoResponse> {    
	match ChatMsgService::<EventModel>::add_msg(&app_state.mongodb.db, &event_id, &new_msg)
		.await
		.map_err(Error::from)
	{
		Ok(res) => Ok(Json(res)),
		Err(e) => Err(e),
	}
}

pub async fn remove_event_msg_handler(
	State(app_state): State<Arc<AppState>>,
	Path((event_id, msg_id)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
	match ChatMsgService::<EventModel>::remove_msg(&app_state.mongodb.db, &event_id, &msg_id)
		.await
		.map_err(Error::from)
	{
		Ok(res) => Ok(Json(res)),
		Err(e) => Err(e),
	}
}

pub async fn update_event_msg_handler(
	State(app_state): State<Arc<AppState>>,
	Path((event_id, msg_id)): Path<(String, String)>,
	Json(update_req): Json<UpdateMsgReq>,
) -> Result<impl IntoResponse> {
	match ChatMsgService::<EventModel>::update_msg(
		&app_state.mongodb.db,
		&event_id,
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
	Path((event_id,)): Path<(String,)>,
) -> Result<impl IntoResponse> {
	let Query(opts) = opts.unwrap_or_default();

	let limit = opts.limit.unwrap_or(10) as i64;
	let page = opts.page.unwrap_or(1) as i64;

	match ChatMsgService::<EventModel>::fetch_msgs(&app_state.mongodb.db, &event_id, limit, page)
		.await
		.map_err(Error::from)
	{
		Ok(res) => Ok(Json(res)),
		Err(e) => Err(e),
	}
}

// TODO: add_chat_to_msg함수 구현된다면..
// pub async fn add_chat_to_event_msg_handler(
//     State(app_state): State<Arc<AppState>>,
//     Path((event_id, msg_id)): Path<(String, String)>,
// ) -> Result<impl IntoResponse> {
//     // Call the add_chat_to_msg function from the repository
//     let messages =
//         ChatMsgService::<EventModel>::add_chat_to_msg(&app_state.mongodb.db, &event_id, &msg_id)
//             .await
//             .unwrap();
//     Json(messages)
// }
