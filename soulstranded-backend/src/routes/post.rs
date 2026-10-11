use axum::Router;
use axum::routing::{get, post};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::handlers::post;
use crate::redis::RedisPool;

pub fn routes() -> Router<(Arc<DatabaseConnection>, RedisPool)> {
    Router::new()
        .route("/publish", post(post::publish))
        .route("/get-posts", get(post::get_posts))
        .route("/like", post(post::like_post))
        .route("/delete", post(post::delete_post))
        .route("/update", post(post::update_post))
        .route("/create-comment", post(post::create_comment))
        .route("/get-comments", get(post::get_comments))
        .route("/delete-comment", post(post::delete_comment))
        .route("/update-comment", post(post::update_comment))
}
