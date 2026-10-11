use axum::Router;
use axum::routing::{get, patch, post};

use crate::handlers::user;
use crate::redis::RedisPool;

pub fn routes() -> Router<(std::sync::Arc<sea_orm::DatabaseConnection>, RedisPool)> {
    Router::new()
        .route("/update-info", patch(user::update_info))
        .route("/upload-avatar", post(user::upload_avatar))
        .route("/statistics", get(user::get_post_statistics))
        .route("/change-password", patch(user::change_password))
}
