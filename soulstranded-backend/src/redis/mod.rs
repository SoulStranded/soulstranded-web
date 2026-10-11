pub mod operations;

use deadpool_redis::{Config, Pool, Runtime};

pub type RedisPool = Pool;

pub async fn connect(redis_url: &str) -> Result<RedisPool, Box<dyn std::error::Error>> {
    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    Ok(pool)
}
