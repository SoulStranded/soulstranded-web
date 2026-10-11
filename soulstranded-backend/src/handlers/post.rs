use std::sync::Arc;

use axum::{Json, extract::Query, extract::State, http::HeaderMap};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::constants;
use crate::database::{CommentInfo, PaginationData, PostInfo};
use crate::handlers::ApiResponse;
use crate::redis::RedisPool;
use crate::services::post;
use crate::utils::verification::extract_bearer_token;

#[derive(Deserialize)]
pub struct PublishRequest {
    pub content: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostQueryParams {
    pub page: u64,
    pub page_size: u64,
    pub user_id: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikePostRequest {
    pub post_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePostRequest {
    pub post_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePostRequest {
    pub post_id: i32,
    pub content: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentRequest {
    pub content: String,
    pub post_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommentsQueryParams {
    pub post_id: i32,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommentRequest {
    pub comment_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommentRequest {
    pub comment_id: i32,
    pub content: String,
}

pub async fn publish(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    Json(request): Json<PublishRequest>,
) -> Json<ApiResponse<()>> {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return Json(ApiResponse::error(
                constants::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            ));
        }
    };

    let publish_res = post::publish_service(&db, token, request.content).await;
    match publish_res {
        Ok(_) => Json(ApiResponse::success(())),
        Err(err_code) => Json(ApiResponse::error(err_code, "Publish failed")),
    }
}

pub async fn get_posts(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    Query(params): Query<PostQueryParams>,
) -> Json<ApiResponse<PaginationData<PostInfo>>> {
    let filter_user_id = params.user_id;

    let current_user_id = match extract_bearer_token(&headers) {
        Some(token) => match crate::utils::verification::verify_login_token(token) {
            Ok(claims) => Some(claims.id),
            Err(_) => None,
        },
        None => None,
    };

    let posts = post::get_posts_service(
        &db,
        params.page,
        params.page_size,
        filter_user_id,
        current_user_id,
    )
    .await;
    match posts {
        Ok(posts) => Json(ApiResponse::success(posts)),
        Err(err_code) => Json(ApiResponse::error(err_code, "Get posts failed")),
    }
}

pub async fn like_post(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    Json(request): Json<LikePostRequest>,
) -> Json<ApiResponse<()>> {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return Json(ApiResponse::error(
                constants::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            ));
        }
    };

    let like_res = post::like_post_service(&db, token, request.post_id).await;
    match like_res {
        Ok(_) => Json(ApiResponse::success(())),
        Err(err_code) => Json(ApiResponse::error(err_code, "Like post failed")),
    }
}

pub async fn delete_post(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    Json(request): Json<DeletePostRequest>,
) -> Json<ApiResponse<()>> {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return Json(ApiResponse::error(
                constants::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            ));
        }
    };

    let delete_res = post::delete_post_service(&db, token, request.post_id).await;
    match delete_res {
        Ok(_) => Json(ApiResponse::success(())),
        Err(err_code) => Json(ApiResponse::error(err_code, "Delete post failed")),
    }
}

pub async fn update_post(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    Json(request): Json<UpdatePostRequest>,
) -> Json<ApiResponse<()>> {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return Json(ApiResponse::error(
                constants::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            ));
        }
    };

    let update_res = post::update_post_service(&db, token, request.post_id, request.content).await;
    match update_res {
        Ok(_) => Json(ApiResponse::success(())),
        Err(err_code) => Json(ApiResponse::error(err_code, "Update post failed")),
    }
}

pub async fn create_comment(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    Json(request): Json<CreateCommentRequest>,
) -> Json<ApiResponse<()>> {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return Json(ApiResponse::error(
                constants::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            ));
        }
    };

    let create_res =
        post::create_comment_service(&db, token, request.content, request.post_id).await;
    match create_res {
        Ok(_) => Json(ApiResponse::success(())),
        Err(err_code) => Json(ApiResponse::error(err_code, "Create comment failed")),
    }
}

pub async fn get_comments(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    Query(params): Query<GetCommentsQueryParams>,
) -> Json<ApiResponse<PaginationData<CommentInfo>>> {
    let comments =
        post::get_comments_service(&db, params.post_id, params.page, params.page_size).await;
    match comments {
        Ok(comments) => Json(ApiResponse::success(comments)),
        Err(err_code) => Json(ApiResponse::error(err_code, "Get comments failed")),
    }
}

pub async fn delete_comment(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    Json(request): Json<DeleteCommentRequest>,
) -> Json<ApiResponse<()>> {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return Json(ApiResponse::error(
                constants::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            ));
        }
    };

    let delete_res = post::delete_comment_service(&db, token, request.comment_id).await;
    match delete_res {
        Ok(_) => Json(ApiResponse::success(())),
        Err(err_code) => Json(ApiResponse::error(err_code, "Delete comment failed")),
    }
}

pub async fn update_comment(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    Json(request): Json<UpdateCommentRequest>,
) -> Json<ApiResponse<()>> {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return Json(ApiResponse::error(
                constants::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            ));
        }
    };

    let update_res =
        post::update_comment_service(&db, token, request.comment_id, request.content).await;
    match update_res {
        Ok(_) => Json(ApiResponse::success(())),
        Err(err_code) => Json(ApiResponse::error(err_code, "Update comment failed")),
    }
}
