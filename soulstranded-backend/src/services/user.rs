use std::env;

use axum::extract::Multipart;
use sea_orm::DatabaseConnection;
use tokio::fs;

use crate::constants;
use crate::database::{
    UserInfo, get_user_post_count, get_user_received_likes_count, update_password_by_email,
    update_user_info_by_email,
};
use crate::handlers::user::{ChangePasswordRequest, UpdateUserInfoRequest};
use crate::utils::verification::{encrypt_password, verify_login_token, verify_password};

pub async fn update_info_service(
    db: &DatabaseConnection,
    token: &str,
    user_info: UpdateUserInfoRequest,
) -> Result<(), i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;
    let email = claims.email;

    update_user_info_by_email(db, &email, user_info)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update user info: {}", e);
            constants::SYSTEM_ERROR
        })?;

    Ok(())
}

/// 验证头像文件的格式和大小
async fn validate_avatar_file(field: axum::extract::multipart::Field<'_>) -> Result<Vec<u8>, i32> {
    // 校验文件格式
    let content_type = field.content_type().ok_or_else(|| {
        tracing::error!("Missing content type");
        constants::VALIDATION_FAILED
    })?;
    let allowed_formats = ["image/jpeg", "image/png", "image/webp"];
    if !allowed_formats.contains(&content_type) {
        tracing::error!("Invalid file format: {}", content_type);
        return Err(constants::VALIDATION_FAILED);
    }

    // 校验文件大小（2MB = 2 * 1024 * 1024 bytes）
    let max_size = 2 * 1024 * 1024;
    let bytes = field.bytes().await.map_err(|e| {
        tracing::error!("Failed to read file bytes: {}", e);
        constants::SYSTEM_ERROR
    })?;
    if bytes.len() > max_size {
        tracing::error!("File size exceeds limit: {} > {}", bytes.len(), max_size);
        return Err(constants::VALIDATION_FAILED);
    }

    Ok(bytes.to_vec())
}

/// 存储头像文件并返回访问 URL
async fn store_avatar_file(bytes: Vec<u8>, email: &str) -> Result<String, i32> {
    // 确保上传目录存在
    let upload_dir =
        env::var("AVATAR_UPLOAD_DIR").unwrap_or_else(|_| "./uploads/avatars".to_string());
    fs::create_dir_all(&upload_dir).await.map_err(|e| {
        tracing::error!("Failed to create upload directory: {}", e);
        constants::SYSTEM_ERROR
    })?;

    // 生成文件名，并且如果文件已经存在，就替换
    let filename = format!("{}.png", email);
    let avatar_path = format!("{}/{}", upload_dir, filename);
    if fs::metadata(&avatar_path).await.is_ok() {
        fs::remove_file(&avatar_path).await.map_err(|e| {
            tracing::error!("Failed to remove existing avatar file: {}", e);
            constants::SYSTEM_ERROR
        })?;
    }
    fs::write(&avatar_path, bytes).await.map_err(|e| {
        tracing::error!("Failed to write avatar file: {}", e);
        constants::SYSTEM_ERROR
    })?;

    Ok(format!("/uploads/avatars/{}", filename))
}

/// 更新用户头像信息
async fn update_user_avatar(
    db: &DatabaseConnection,
    email: &str,
    avatar_url: &str,
) -> Result<UserInfo, i32> {
    update_user_info_by_email(
        db,
        email,
        UpdateUserInfoRequest {
            username: None,
            avatar: Some(avatar_url.to_string()),
        },
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to update user info: {}", e);
        constants::SYSTEM_ERROR
    })
}

pub async fn upload_avatar_service(
    db: &DatabaseConnection,
    token: &str,
    mut multipart: Multipart,
) -> Result<UserInfo, i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;
    let email = claims.email;

    // 处理 multipart 数据，包括文件上传
    let mut user_info = None;
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Failed to read multipart field: {}", e);
        constants::SYSTEM_ERROR
    })? {
        if let Some(name) = field.name() {
            if name == "avatar" {
                // 验证头像文件
                let bytes = validate_avatar_file(field).await?;
                // 处理文件存储
                let avatar_url = store_avatar_file(bytes, &email).await?;
                // 更新用户信息并返回最新的用户信息
                user_info = Some(update_user_avatar(db, &email, &avatar_url).await?);
                break;
            }
        }
    }

    if user_info.is_none() {
        tracing::error!("No avatar field found or user not found");
        return Err(constants::VALIDATION_FAILED);
    }

    Ok(user_info.unwrap())
}

pub async fn get_post_statistics_service(
    db: &DatabaseConnection,
    token: &str,
) -> Result<(u64, u64), i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;

    let post_count = get_user_post_count(db, claims.id).await.map_err(|e| {
        tracing::error!("Failed to get user post count: {}", e);
        constants::SYSTEM_ERROR
    })?;

    let likes_received = get_user_received_likes_count(db, claims.id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get user received likes count: {}", e);
            constants::SYSTEM_ERROR
        })?;

    Ok((post_count, likes_received))
}

pub async fn change_password_service(
    db: &DatabaseConnection,
    token: &str,
    request: ChangePasswordRequest,
) -> Result<(), i32> {
    let claims = verify_login_token(token).map_err(|_| constants::TOKEN_INVALID)?;
    let email = claims.email;

    let verify_result = verify_password(db, &email, &request.old_password).await;
    if verify_result != constants::SUCCESS {
        return Err(verify_result);
    }

    let encrypted_password = encrypt_password(&request.new_password);
    update_password_by_email(db, &email, &encrypted_password)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update password: {}", e);
            constants::SYSTEM_ERROR
        })?;

    Ok(())
}
