use std::env;

use dotenvy::dotenv;

/// 表示服务器配置的结构体
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// 表示数据库配置的结构体
pub struct DatabaseConfig {
    pub database_url: String,
}

/// 表示 Redis 配置的结构体
pub struct RedisConfig {
    pub redis_url: String,
}

/// 表示应用配置的结构体
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
}

/// 从环境变量加载应用配置
impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        // 加载.env文件
        dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in .env file");

        Ok(AppConfig {
            server: ServerConfig { host, port },
            database: DatabaseConfig { database_url },
            redis: RedisConfig { redis_url },
        })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    pub fn database_url(&self) -> &str {
        &self.database.database_url
    }

    pub fn redis_url(&self) -> &str {
        &self.redis.redis_url
    }
}
