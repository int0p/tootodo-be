use std::sync::Arc;

use axum::{middleware, Router};

use super::handler::{
    category::category_router, event::event_router, habit::habit_router, memo::memo_router,
    task::task_router,
};
use crate::{auth::utils::auth::auth_request, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(category_router(app_state.clone()))
        .merge(event_router(app_state.clone()))
        .merge(habit_router(app_state.clone()))
        .merge(memo_router(app_state.clone()))
        .merge(task_router(app_state.clone()))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_request,
        ))
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
