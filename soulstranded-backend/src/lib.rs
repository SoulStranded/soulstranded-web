pub mod config;
pub mod constants;
pub mod database;
pub mod entity;
pub mod handlers;
pub mod middleware;
pub mod redis;
pub mod routes;
pub mod services;
pub mod utils;

use axum::middleware::from_fn;

use middleware::request_logger;

pub async fn create_app() -> Result<axum::Router, Box<dyn std::error::Error>> {
    let config = config::AppConfig::from_env().expect("Failed to load configuration");

    let db = database::connect(config.database_url())
        .await
        .expect("Failed to connect to database");
    tracing::info!("Database connected");

    let redis_conn = redis::connect(config.redis_url())
        .await
        .expect("Failed to connect to Redis");
    tracing::info!("Redis connected");

    let app = routes::create_router(db, redis_conn).layer(from_fn(request_logger::log_requests));

    Ok(app)
}
