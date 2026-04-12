pub mod auth;
pub mod post;
pub mod user;

use axum::Router;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::redis::RedisPool;

use axum::routing::get_service;
use tower_http::services::ServeDir;

pub fn create_router(db: DatabaseConnection, redis: RedisPool) -> Router {
    let db = Arc::new(db);

    Router::new()
        .nest("/auth", auth::routes())
        .nest("/post", post::routes())
        .nest("/user", user::routes())
        .nest_service("/uploads", get_service(ServeDir::new("./uploads")))
        .with_state((db, redis))
}
