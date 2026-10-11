use axum::Router;
use axum::routing::post;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::handlers::auth;
use crate::redis::RedisPool;

pub fn routes() -> Router<(Arc<DatabaseConnection>, RedisPool)> {
    Router::new()
        .route("/captcha", post(auth::send_captcha))
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/reset-password", post(auth::reset_password))
}
