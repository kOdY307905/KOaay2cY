use axum::extract::Multipart;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router, Server};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::services::ServeDir;
use tower_http::services::ServeDirConfig;
use tower_http::trace::TraceLayer;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;
use uuid::Uuid;

// 配置日志记录
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::Layer::default().event_format(fmt::format::Format::default()
            .with_level(false)
            .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339())
        ))
        .init();

    // 设置HTTP服务器监听的地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // 设置静态文件服务
    let static_files = ServeDir::new(ServeDirConfig::new("./www").index_file("index.html"));

    // 创建HTTP路由
    let app = Router::new()
        .route("/", get(index))
        .nest("/upload", get(upload.html).handle_error(handle_error))
        .layer(TraceLayer::new_for_subscriber(
            &tokio::spawn(tracing_appender::non_blocking::Worker::new(())),
        ));

    // 启动HTTP服务器
    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// 首页路由处理器
async fn index() -> impl IntoResponse {
    Html(include_str!("./www/index.html"))
}

// 文件上传路由处理器
async fn upload() -> impl IntoResponse {
    Html(include_str!("./www/upload.html"))
}

// 错误处理器
async fn handle_error(error: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal server error: {}", error))
}

// 捕获系统信号以优雅关闭服务器
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to listen for event")
    };

    let terminate = async {
        signal::shutdown().await.expect("failed to listen for event")
    };

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received SIGINT (Ctrl+C), shutting down");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM, shutting down");
        },
    }
}

// 使用AXUM的Multipart处理器解析上传的文件
async fn upload_file.multipart(multipart: Multipart) -> impl IntoResponse {
    let form = multipart
        .map(|item| item.map(|part| part.unwrap()))
        .collect::<Result<Vec<_>, _>>()
        .await;

    if let Err(e) = form {
        return (StatusCode::BAD_REQUEST, format!("Failed to parse multipart: {}", e));
    }

    let files = form.unwrap().iter()
        .filter_map(|field| field.file())
        .map(|file| file.into_inner().path().display().to_string())
        .collect::<Vec<_>>();

    let response_body = format!("Received {} files.", files.len());
    (StatusCode::OK, response_body)
}
