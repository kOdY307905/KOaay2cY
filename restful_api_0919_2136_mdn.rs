use warp::Filter;

// 引入warp库中的Rejection和Reply类型，用于错误处理和响应
use warp::{Rejection, Reply};

// 引入tokio库，用于异步运行环境
#[tokio::main]
async fn main() {
    // 定义一个简单的GET请求处理函数
    let hello_route = warp::path("hello")
        .and(warp::get())
        .map(|| warp::reply::json(&"Hello, World!"));

    // 定义一个更复杂的GET请求处理函数，带参数
    let sum_route = warp::path("sum")
        .and(warp::get())
        .and(warp::query::<(warp::filters::extract::UnwrapFilter<u32>, warp::filters::extract::UnwrapFilter<u32>)>())
        .map(sum);

    // 启动服务器，监听3000端口
    warp::serve(hello_route.or(sum_route))
        .run(([127, 0, 0, 1], 3000))
        .await;
}

// 定义一个求和函数，用于处理/sum路径的GET请求
async fn sum((a, b): (u32, u32)) -> Result<impl Reply, Rejection> {
    // 进行简单的错误处理，确保输入值有效
    if a == 0 || b == 0 {
        return Err(warp::reject::custom(sum_error()));
    }

    // 返回计算结果
    Ok(warp::reply::json(&a + b))
}

// 定义一个自定义错误类型，用于处理求和错误
#[derive(Debug)]
struct SumError;

// 实现warp::reject::Reject trait，以便我们的自定义错误可以被warp使用
impl warp::reject::Reject for SumError {}

// 实现错误信息的展示
impl std::fmt::Display for SumError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Non-zero values required for both a and b")
    }
}

// 实现错误信息的描述
impl std::error::Error for SumError {}
