use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use tokio;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};
use warp::reject::Reject;

// 定义一个错误类型，用于表单验证失败时返回
#[derive(Debug)]
struct ValidationErrors(Vec<String>);

impl Reject for ValidationErrors {}

// 定义一个输入表单数据的结构体
#[derive(Deserialize)]
struct FormData {
    username: String,
    email: String,
    age: u8,
}

// 实现表单验证逻辑
async fn validate_form(data: FormData) -> Result<FormData, ValidationErrors> {
    // 验证用户名是否为空
    if data.username.trim().is_empty() {
        return Err(ValidationErrors(vec!["Username cannot be empty".to_string()]));
    }
    // 验证邮箱是否符合规范
    if !data.email.contains('@') {
        return Err(ValidationErrors(vec!["Email is invalid".to_string()]));
    }
    // 验证年龄是否在合理范围内
    if data.age < 18 || data.age > 100 {
        return Err(ValidationErrors(vec!["Age is out of valid range".to_string()]));
    }
    // 如果所有验证通过，返回验证后的FormData
    Ok(data)
}

// 创建一个warp filter，用于处理POST请求并验证表单数据
fn form_validator() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("validate"))
        .and(warp::body::json())
        .and_then(|data: FormData| async move {
            match validate_form(data).await {
                Ok(_) => Ok(warp::reply::json(&{"message": "Validation passed"}
                )),
                Err(err) => Ok(warp::reply::with_status(
                    warp::reply::json(&{"errors": err.0}
                    ),
                    StatusCode::BAD_REQUEST,
                )),
            }
        })
}

#[tokio::main]
async fn main() {
    // 启动warp服务器
    warp::serve(form_validator()).run(([127, 0, 0, 1], 3030)).await;
}