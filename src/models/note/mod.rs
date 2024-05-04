pub mod controller;
pub mod error;
pub mod handler;
pub mod model;
mod response;

use std::sync::Arc;

use axum::{
    routing::{get, post}, Router
};

use self::handler::{
    create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
    note_list_handler,
};
use crate::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/notes/", post(create_note_handler))
        .route("/api/notes", get(note_list_handler))
        .route(
            "/api/notes/:id",
            get(get_note_handler)
                .patch(edit_note_handler)
                .delete(delete_note_handler),
        )
        .with_state(app_state)
}
