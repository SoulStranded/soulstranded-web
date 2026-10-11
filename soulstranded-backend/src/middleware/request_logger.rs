use std::time::Instant;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

/// 请求日志记录中间件
/// 记录请求方法、路径和响应耗时
pub async fn log_requests(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let start_time = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();

    // 处理请求
    let response = next.run(request).await;

    // 计算耗时
    let duration = start_time.elapsed();

    // 记录日志
    tracing::info!(
        "method={} uri={} status={} duration={}ms",
        method,
        uri,
        response.status(),
        duration.as_millis()
    );

    Ok(response)
}
