pub mod controller;
pub mod handler;
pub mod model;
mod response;
pub mod schema;
use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use self::handler::{
    create_habit_handler, delete_habit_handler, get_habit_handler, habit_list_handler,
    update_habit_handler,
};
use crate::{auth::utils::auth::auth_request, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/habits/", post(create_habit_handler))
        .route("/api/habits", get(habit_list_handler))
        .route(
            "/api/habits/:id",
            get(get_habit_handler)
                .patch(update_habit_handler)
                .delete(delete_habit_handler),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_request,
        ))
        .with_state(app_state)
}
