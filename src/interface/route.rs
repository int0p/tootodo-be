use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post, put, delete, patch},
    Router,
};

use super::handler::{memo::*,event::*};
use crate::{auth::utils::auth::auth_request, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(memo_router(app_state.clone()))
        .merge(event_router(app_state.clone()))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_request,
        ))
        .with_state(app_state)
}

pub fn memo_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/memos/", post(create_memo_handler))
        .route("/api/memos", get(memo_list_handler))
        .route(
            "/api/memos/:id",
            get(get_memo_handler)
                .patch(update_memo_handler)
                .delete(delete_memo_handler),
        )        
}

pub fn event_router(app_state: Arc<AppState>)->Router{
    Router::new()
        .route("/api/events/", post(create_event_handler))
        .route("/api/events", get(event_list_handler))
        .route(
            "/api/events/:id",
            get(get_event_handler)
                .patch(update_event_handler)
                .delete(delete_event_handler),
        )
}

pub fn schedule_router(app_state: Arc<AppState>)->Router{
    Router::new()
    .route("/schedule/:user_id", get(fetch_schedule))
    .route("/schedule", post(create_schedule))
    .route("/schedule/add_item/:user_id", put(add_item_to_schedule))
    .route("/schedule/remove_item/:user_id", put(remove_item_from_schedule))
    .route("/schedule/update_item/:user_id", put(update_item_in_schedule))
    .route("/schedule/reset/:user_id", delete(reset_schedule))
}