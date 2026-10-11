use std::env;

use axum::http::HeaderMap;
use axum::http::header::AUTHORIZATION;
use bcrypt;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::seq::IndexedRandom;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::constants;
use crate::database::{UserInfo, get_password_by_email};
use crate::redis::RedisPool;
use crate::redis::operations::{delete_key, get_string};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    username: String,
    pub email: String,
    exp: i64,
    iat: i64,
}

/// 生成一个6位随机验证码
pub fn generate_captcha() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let mut rng = rand::rng();
    String::from_utf8((0..6).map(|_| *CHARSET.choose(&mut rng).unwrap()).collect()).unwrap()
}

/// 创建SMTP传输器
fn create_smtp_transport() -> SmtpTransport {
    let smtp_url = env::var("SMTP_URL").expect("SMTP_URL must be set in .env file");
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set in .env file");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set in .env file");

    let creds = Credentials::new(smtp_username, smtp_password);
    SmtpTransport::relay(&smtp_url)
        .unwrap()
        .credentials(creds)
        .build()
}

/// 构建验证码邮件
fn build_verification_email(receiver_email: String, code: String) -> Message {
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set in .env file");

    let text_body = format!(
        r#"【SoulStranded】邮箱验证码

尊敬的用户：

您好！您正在进行邮箱验证操作，您的验证码如下：

{}

验证码有效期为10分钟，请在有效期内完成验证。

安全提示：
• 请勿将验证码告知他人
• 如非本人操作，请忽略此邮件
• 如有疑问，请联系客服

此邮件由系统自动发送，请勿回复。

"#,
        code
    );

    Message::builder()
        .from(smtp_username.parse().unwrap())
        .to(receiver_email.parse().unwrap())
        .subject("【SoulStranded】邮箱验证码")
        .body(text_body)
        .unwrap()
}

/// 连接smtp服务器给邮箱发送验证码
pub fn send_captcha(receiver_email: String, code: String) -> bool {
    let mailer = create_smtp_transport();
    let email = build_verification_email(receiver_email, code);

    match mailer.send(&email) {
        Ok(_) => {
            tracing::info!("Captcha sent successfully");
            true
        }
        Err(e) => {
            tracing::error!("Failed to send captcha: {}", e);
            false
        }
    }
}

/// 验证密码是否匹配
pub fn verify_password_match(password: &str, confirm_password: &str) -> bool {
    if password != confirm_password {
        tracing::warn!("Password and confirm password do not match");
        false
    } else {
        true
    }
}

/// 验证验证码是否正确
pub async fn verify_captcha(redis: RedisPool, email: &str, input_captcha: &str) -> bool {
    let key = format!("captcha:{}", email);

    let stored_code = match get_string(&redis, &key).await {
        Ok(Some(code)) => code,
        _ => return false, // 包括 Err 或 None
    };
    if stored_code != input_captcha {
        return false;
    }

    let _ = delete_key(&redis, &key).await;

    true
}

/// 对密码加密
pub fn encrypt_password(password: &str) -> String {
    match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
        Ok(hashed_password) => hashed_password,
        Err(e) => {
            tracing::error!("Failed to hash password: {}", e);
            password.to_string()
        }
    }
}

/// 验证密码是否匹配
pub async fn verify_password(db: &DatabaseConnection, email: &str, password: &str) -> i32 {
    let stored_password = get_password_by_email(db, email).await;
    match stored_password {
        Ok(Some(p)) => match bcrypt::verify(password, &p) {
            Ok(true) => constants::SUCCESS,
            Ok(false) => constants::PASSWORD_ERROR,
            Err(e) => {
                tracing::error!("Failed to verify password: {}", e);
                constants::PASSWORD_ERROR
            }
        },
        Ok(None) => constants::USER_NOT_FOUND,
        Err(e) => {
            tracing::error!("Failed to get password from database: {}", e);
            constants::USER_NOT_FOUND
        }
    }
}

fn get_expiration_hours(remember: bool) -> i64 {
    let env_key = if remember {
        "JWT_REMEMBER_EXPIRATION_HOURS"
    } else {
        "JWT_EXPIRATION_HOURS"
    };

    let default_hours = if remember { 720 } else { 24 };

    env::var(env_key)
        .unwrap_or_else(|_| default_hours.to_string())
        .parse::<i64>()
        .unwrap_or(default_hours)
}

/// 通过用户信息生成登录令牌
pub fn generate_login_token(user_info: &UserInfo, remember_me: bool) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");
    let expiration_hours = get_expiration_hours(remember_me);

    let now = Utc::now();
    let exp = now + Duration::hours(expiration_hours);

    let claims = Claims {
        id: user_info.id,
        username: user_info.username.clone(),
        email: user_info.email.clone(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap_or_else(|_| {
        tracing::error!("Failed to generate JWT token");
        String::new()
    })
}

/// 验证登录令牌
pub fn verify_login_token(token: &str) -> Result<Claims, i32> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(token, &decoding_key, &validation);
    match token_data {
        Ok(t) => Ok(t.claims),
        Err(e) => {
            tracing::error!("Failed to decode JWT token: {}", e);
            Err(constants::TOKEN_INVALID)
        }
    }
}

/// 从请求头中提取 Bearer Token
pub fn extract_bearer_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .filter(|t| !t.is_empty())
}
