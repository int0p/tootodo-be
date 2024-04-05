mod config;
mod error;
mod handler;
mod model;
mod response;
mod route;
mod utils;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use config::Config;
use dotenv::dotenv;
// use redis::Client;
use route::create_router;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::health_checker_handler,
        handler::register_user_handler,
        handler::login_user_handler,
        handler::logout_handler,
        handler::get_me_handler,
    ),
    components(
        schemas(model::FilterUser,model::UserData,model::UserResponse,model::RegisterUserSchema, model::LoginUserSchema,error::ErrorResponse,model::LoginUserResponse),
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
pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: Config,
    // pub redis_client: Client,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // let redis_client = match Client::open(config.redis_url.to_owned()) {
    //     Ok(client) => {
    //         println!("✅Connection to the redis is successful!");
    //         client
    //     }
    //     Err(e) => {
    //         println!("🔥 Error connecting to Redis: {}", e);
    //         std::process::exit(1);
    //     }
    // };

    let cors = CorsLayer::new()
        .allow_origin("https://tootodo.life/*".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:8000")
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
        // redis_client: redis_client.clone(),
    }))
    .merge(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    .layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
