use std::io;

use deadpool_redis::PoolError;
use deadpool_redis::redis::{AsyncCommands, RedisError};

use super::RedisPool;

/// 将 PoolError 转换为 RedisError 的辅助函数
fn pool_error_to_redis_error(pool_err: PoolError) -> RedisError {
    match pool_err {
        PoolError::Backend(redis_err) => redis_err,
        PoolError::Timeout(_) => {
            // 转为 io::Error 再 into RedisError
            io::Error::new(io::ErrorKind::TimedOut, "Redis operation timed out").into()
        }
        PoolError::Closed => {
            io::Error::new(io::ErrorKind::Other, "Redis connection pool closed").into()
        }
        _ => io::Error::new(
            io::ErrorKind::Other,
            format!("Unexpected Redis pool error: {:?}", pool_err),
        )
        .into(),
    }
}

/// 设置字符串键值对，带过期时间（秒）
pub async fn set_ex_string(
    pool: &RedisPool,
    key: &str,
    value: &str,
    expire_seconds: u64,
) -> Result<(), RedisError> {
    let mut conn = pool.get().await.map_err(pool_error_to_redis_error)?;
    conn.set_ex(key, value, expire_seconds).await
}

/// 获取字符串值，key 不存在时返回 None
pub async fn get_string(pool: &RedisPool, key: &str) -> Result<Option<String>, RedisError> {
    let mut conn = pool.get().await.map_err(pool_error_to_redis_error)?;
    conn.get(key).await
}

/// 删除 key，返回删除的 key 数量
pub async fn delete_key(pool: &RedisPool, key: &str) -> Result<u64, RedisError> {
    let mut conn = pool.get().await.map_err(pool_error_to_redis_error)?;
    conn.del(key).await
}
