use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::constants;
use crate::database::UserInfo;
use crate::handlers::ApiResponse;
use crate::redis::RedisPool;
use crate::services::auth;

#[derive(Deserialize)]
pub struct CaptchaRequest {
    pub email: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordWithConfirm {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub captcha: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub username: String,
    #[serde(flatten)]
    pub password_with_confirm: PasswordWithConfirm,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

pub async fn send_captcha(
    State((_db, redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    Json(request): Json<CaptchaRequest>,
) -> Json<ApiResponse<()>> {
    let send_res = auth::captcha_service(redis, request.email).await;

    match send_res {
        constants::SUCCESS => Json(ApiResponse::success(())),
        _ => Json(ApiResponse::error(send_res, "Captcha send failed")),
    }
}

pub async fn register(
    State((db, redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    Json(request): Json<RegisterRequest>,
) -> Json<ApiResponse<()>> {
    let register_res = auth::register_service(&db, redis, request).await;

    match register_res {
        constants::SUCCESS => Json(ApiResponse::success(())),
        _ => Json(ApiResponse::error(register_res, "Register failed")),
    }
}

pub async fn login(
    State((db, _redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    Json(request): Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    let login_res = auth::login_service(&db, request).await;

    if let Some(login_res) = login_res {
        Json(ApiResponse::success(login_res))
    } else {
        Json(ApiResponse::error(
            constants::PASSWORD_ERROR,
            "Login failed",
        ))
    }
}

pub async fn reset_password(
    State((db, redis)): State<(Arc<DatabaseConnection>, RedisPool)>,
    Json(request): Json<PasswordWithConfirm>,
) -> Json<ApiResponse<()>> {
    let reset_res = auth::reset_password_service(&db, redis, request).await;

    match reset_res {
        constants::SUCCESS => Json(ApiResponse::success(())),
        _ => Json(ApiResponse::error(reset_res, "Reset password failed")),
    }
}
