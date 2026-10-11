use sea_orm::DatabaseConnection;

use crate::constants;
use crate::database::{
    NewUser, add_user, check_email_registered, get_user_info_by_email, update_password_by_email,
};
use crate::handlers::auth::{LoginRequest, LoginResponse, PasswordWithConfirm, RegisterRequest};
use crate::redis::RedisPool;
use crate::redis::operations::set_ex_string;
use crate::utils::verification;

pub async fn captcha_service(redis: RedisPool, email: String) -> i32 {
    let code = verification::generate_captcha();
    tracing::info!("Generated captcha: {}", code);

    let key = format!("captcha:{}", email);
    if let Err(e) = set_ex_string(&redis, &key, &code, 600).await {
        tracing::error!("Failed to store captcha in Redis: {}", e);
        return constants::SYSTEM_ERROR;
    }
    tracing::info!("Captcha stored in Redis for email: {}", email);

    if verification::send_captcha(email, code) {
        constants::SUCCESS
    } else {
        constants::SYSTEM_ERROR
    }
}

async fn save_user(
    db: &DatabaseConnection,
    username: String,
    email: String,
    password: String,
) -> i32 {
    match add_user(
        db,
        NewUser {
            username,
            email,
            password,
        },
    )
    .await
    {
        Ok(_) => constants::SUCCESS,
        Err(_) => constants::SYSTEM_ERROR,
    }
}

pub async fn register_service(
    db: &DatabaseConnection,
    redis: RedisPool,
    request: RegisterRequest,
) -> i32 {
    let RegisterRequest {
        username,
        password_with_confirm,
    } = request;
    let PasswordWithConfirm {
        email,
        password,
        confirm_password,
        captcha,
    } = password_with_confirm;

    if check_email_registered(db, &email).await.unwrap_or(false) {
        return constants::EMAIL_EXISTS;
    }
    if !verification::verify_password_match(&password, &confirm_password) {
        return constants::VALIDATION_FAILED;
    }
    if !verification::verify_captcha(redis, &email, &captcha).await {
        return constants::CAPTCHA_ERROR;
    }

    let encrypted_password = verification::encrypt_password(&password);
    save_user(db, username, email, encrypted_password).await
}

pub async fn login_service(
    db: &DatabaseConnection,
    request: LoginRequest,
) -> Option<LoginResponse> {
    let LoginRequest {
        email,
        password,
        remember_me,
    } = request;

    let verification_result = verification::verify_password(db, &email, &password).await;
    match verification_result {
        constants::SUCCESS => {
            let user_info = get_user_info_by_email(db, &email).await.unwrap_or(None);

            if let Some(user_info) = user_info {
                let login_token = verification::generate_login_token(&user_info, remember_me);
                Some(LoginResponse {
                    token: login_token,
                    user: user_info,
                })
            } else {
                None
            }
        }
        _ => return None,
    }
}

pub async fn reset_password_service(
    db: &DatabaseConnection,
    redis: RedisPool,
    request: PasswordWithConfirm,
) -> i32 {
    let PasswordWithConfirm {
        email,
        password,
        confirm_password,
        captcha,
    } = request;

    if !check_email_registered(db, &email).await.unwrap_or(false) {
        return constants::USER_NOT_FOUND;
    }
    if !verification::verify_password_match(&password, &confirm_password) {
        return constants::VALIDATION_FAILED;
    }
    if !verification::verify_captcha(redis, &email, &captcha).await {
        return constants::CAPTCHA_ERROR;
    }

    let encrypted_password = verification::encrypt_password(&password);
    match update_password_by_email(db, &email, &encrypted_password).await {
        Ok(_) => constants::SUCCESS,
        Err(_) => constants::SYSTEM_ERROR,
    }
}
