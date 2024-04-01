use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post,delete},
    Router,
};

use crate::{
    handler::{
        get_me_handler, health_checker_handler, login_user_handler, logout_handler,
        refresh_access_token_handler, register_user_handler, google_oauth_handler
    },
    utils::auth::auth_request,
    AppState,
};


pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register_user_handler))
        .route("/api/auth/login", post(login_user_handler))
        .route("/api/auth/refresh", get(refresh_access_token_handler))
        .route("/api/sessions/oauth/google",get(google_oauth_handler))
        .route(
            "/api/auth/logout",
            delete(logout_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth_request)),
        )
        .route(
            "/api/users/me",
            get(get_me_handler)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth_request)),
        )
        .with_state(app_state)

}
