use tokio::time::{sleep, Duration};
use std::error::Error;
use std::time::Instant;
use serde::{Serialize, Deserialize};
use serde_json::Result as JsonResult;
use warp::Filter;

// 定义环境监测数据结构
#[derive(Serialize, Deserialize, Debug)]
struct EnvironmentData {
    temperature: f64,
    humidity: f64,
    timestamp: Instant,
}

// 实现获取环境数据的异步函数
async fn get_environment_data() -> Result<EnvironmentData, Box<dyn Error>> {
    let temperature = rand::random::<f64>() * 50.0;
    let humidity = rand::random::<f64>() * 100.0;
    let timestamp = Instant::now();

    Ok(EnvironmentData {
        temperature,
        humidity,
        timestamp,
    })
}

// 实现将环境数据转换为JSON的函数
async fn to_json(env_data: EnvironmentData) -> JsonResult<String> {
    serde_json::to_string(&env_data)
}

// 实现HTTP服务的函数
async fn run_server() -> Result<(), Box<dyn Error>> {
    let get_env_data = warp::path("env")
        .and_then(|| async {
            // 获取环境数据
            let env_data = get_environment_data().await?;
            // 转换为JSON
            let json_str = to_json(env_data).await?;
            Ok::<_, warp::Rejection>(json_str)
        });

    warp::serve(get_env_data).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 运行HTTP服务
    run_server().await
}
