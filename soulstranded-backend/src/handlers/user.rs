use std::sync::Arc;

use axum::{
    Json,
    extract::{Multipart, State},
    http::HeaderMap,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::handlers::ApiResponse;
use crate::redis::RedisPool;
use crate::services::user;
use crate::utils::verification::extract_bearer_token;
use crate::{constants, database::UserInfo};

#[derive(Deserialize)]
pub struct UpdateUserInfoRequest {
    pub username: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostStatisticsResponse {
    pub post_count: u64,
    pub likes_received: u64,
}

pub async fn update_info(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    Json(request): Json<UpdateUserInfoRequest>,
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

    let update_res = user::update_info_service(&db, token, request).await;
    match update_res {
        Ok(_) => Json(ApiResponse::success(())),
        Err(err_code) => Json(ApiResponse::error(err_code, "Update info failed")),
    }
}

pub async fn upload_avatar(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    multipart: Multipart,
) -> Json<ApiResponse<UserInfo>> {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return Json(ApiResponse::error(
                constants::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            ));
        }
    };

    let upload_res = user::upload_avatar_service(&db, token, multipart).await;
    match upload_res {
        Ok(user_info) => Json(ApiResponse::success(user_info)),
        Err(err_code) => Json(ApiResponse::error(err_code, "Upload avatar failed")),
    }
}

pub async fn get_post_statistics(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
) -> Json<ApiResponse<PostStatisticsResponse>> {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return Json(ApiResponse::error(
                constants::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            ));
        }
    };

    let result = user::get_post_statistics_service(&db, token).await;
    match result {
        Ok(stats) => Json(ApiResponse::success(PostStatisticsResponse {
            post_count: stats.0,
            likes_received: stats.1,
        })),
        Err(err_code) => Json(ApiResponse::error(err_code, "Get post statistics failed")),
    }
}

pub async fn change_password(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    headers: HeaderMap,
    Json(request): Json<ChangePasswordRequest>,
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

    let result = user::change_password_service(&db, token, request).await;
    match result {
        Ok(_) => Json(ApiResponse::success(())),
        Err(err_code) => Json(ApiResponse::error(err_code, "Change password failed")),
    }
}
