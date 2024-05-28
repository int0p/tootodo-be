use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

use super::handler::{event::*, habit::*, memo::*};
use crate::{auth::utils::auth::auth_request, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(memo_router(app_state.clone()))
        .merge(event_router(app_state.clone()))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_request,
        ))
}

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
        .route("/events/:event_id/chat/", post(add_msg_handler))
        .route("/events/:event_id/chat", get(fetch_msgs_handler))
        .route(
            "/events/:event_id/chat/:msg_id",
            get(get_msg_handler)
                .delete(remove_msg_handler)
                .put(update_msg_handler),
        )
        // .route(
        //     "/events/:event_id/chat/:msg_id/add_chat",
        //     post(add_chat_to_msg_handler),
        // )
        .with_state(app_state)
}

pub fn habit_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/habits/", post(create_habit_handler))
        .route("/api/habits", get(habit_list_handler))
        .route(
            "/api/habits/:id",
            get(get_habit_handler)
                .patch(update_habit_handler)
                .delete(delete_habit_handler),
        )
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
        .with_state(app_state)
}

// pub fn schedule_router(app_state: Arc<AppState>) -> Router {
//     Router::new()
//         .route("/schedule/:user_id", get(fetch_schedule))
//         .route("/schedule", post(create_schedule))
//         .route("/schedule/add_item/:user_id", put(add_item_to_schedule))
//         .route(
//             "/schedule/remove_item/:user_id",
//             put(remove_item_from_schedule),
//         )
//         .route(
//             "/schedule/update_item/:user_id",
//             put(update_item_in_schedule),
//         )
//         .route("/schedule/reset/:user_id", delete(reset_schedule))
//         .with_state(app_state)
// }
