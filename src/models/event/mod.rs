pub mod controller;
pub mod handler;
pub mod model;
pub mod response;
pub mod schema;

use std::sync::Arc;

use axum::{
    middleware, routing::{get, post}, Router
};

use self::handler::{
    create_event_handler, delete_event_handler, update_event_handler, get_event_handler,
    event_list_handler,
};
use crate::{auth::utils::auth::auth_request, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/events/", post(create_event_handler))
        .route("/api/events", get(event_list_handler))
        .route(
            "/api/events/:id",
            get(get_event_handler)
                .patch(update_event_handler)
                .delete(delete_event_handler),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_request,
        ))        
        .with_state(app_state)
}