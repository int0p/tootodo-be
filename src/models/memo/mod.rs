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
    create_memo_handler, delete_memo_handler, get_memo_handler, memo_list_handler,
    update_memo_handler,
};
use crate::{auth::utils::auth::auth_request, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/memos/", post(create_memo_handler))
        .route("/api/memos", get(memo_list_handler))
        .route(
            "/api/memos/:id",
            get(get_memo_handler)
                .patch(update_memo_handler)
                .delete(delete_memo_handler),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_request,
        ))
        .with_state(app_state)
}
