use std::net::SocketAddr;

use tracing_subscriber::fmt::time::ChronoLocal;

use soulstranded_backend::create_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
        .init();

    // 构建应用
    let app = create_app().await?;

    // 获取地址（这里需要从 config 读取，所以 config 必须 pub）
    let config =
        soulstranded_backend::config::AppConfig::from_env().expect("Failed to load configuration");
    let addr: SocketAddr = config.address().parse().expect("Invalid address");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Server running on http://{}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
