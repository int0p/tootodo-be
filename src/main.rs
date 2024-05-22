mod auth;
mod config;
mod ctx;
mod db;
mod error;
mod models;

use axum::http::{HeaderMap, Request, Response};
use axum::{
    body::Body,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    Router,
};
use bytes::Bytes;
use config::Config;
use db::{MongoDB, DB};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use std::time::Duration;
use tower_http::{classify::ServerErrorsFailureClass, cors::CorsLayer, trace::TraceLayer};
use tracing::Span;
use utoipa::OpenApi;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

pub use self::error::{Error, Result};

pub struct AppState {
    pub db: Pool<Postgres>,
    pub mongodb: MongoDB,
    pub env: Config,
    // pub redis_client: Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "dev" {
        dotenv::from_filename(".env.dev").ok();
    } else {
        dotenv().ok();
    }

    tracing_subscriber::fmt()
        .with_max_level(tracing::level_filters::LevelFilter::DEBUG)
        .without_time() // For early local development.
        .with_target(false)
        // .with_env_filter(EnvFilter::from_default_env())
        .init();

    let origins = [
        "https://tootodo.life"
            .parse::<HeaderValue>()
            .map_err(|e| Error::HeaderError(e))?,
        "http://localhost:5173"
            .parse::<HeaderValue>()
            .map_err(|e| Error::HeaderError(e))?,
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let config = Config::init();
    let postgredb = DB::init().await?;
    let mongodb = MongoDB::init().await?;

    let app_state = Arc::new(AppState {
        db: postgredb.db.clone(),
        mongodb: mongodb.clone(),
        env: config.clone(),
    });
    let app = Router::new()
        .merge(auth::create_router(app_state.clone()))
        .merge(models::memo::create_router(app_state.clone()))
        .merge(models::event::create_router(app_state.clone()))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| tracing::debug_span!("http-request"))
                .on_request(|request: &Request<Body>, _span: &Span| {
                    tracing::debug!("started {} {}", request.method(), request.uri().path())
                })
                .on_response(
                    |response: &Response<Body>, latency: Duration, _span: &Span| {
                        tracing::debug!("response generated in {:?}", latency)
                    },
                )
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _span: &Span| {
                    tracing::debug!("sending {} bytes", chunk.len())
                })
                .on_eos(
                    |trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                        tracing::debug!("stream closed after {:?}", stream_duration)
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                        tracing::debug!("something went wrong")
                    },
                ),
        )
        .layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .map_err(|e| Error::ServerError(e))?;

    Ok(axum::serve(listener, app)
        .await
        .map_err(|e| Error::ServerError(e))?)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::handler::health_checker_handler,
        auth::handler::register_user_handler,
        auth::handler::login_user_handler,
        auth::handler::logout_handler,
        auth::handler::get_me_handler,
    ),
    components(
        schemas(auth::model::FilterUser,auth::model::UserData,auth::model::UserResponse,auth::model::RegisterUserSchema, auth::model::LoginUserSchema,
           error::ErrorResponse,auth::model::LoginUserResponse),
    ),
    tags(
        (name = "Rust REST API", description = "Authentication in Rust Endpoints")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
