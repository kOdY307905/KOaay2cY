// data_validator.rs

use serde::Deserialize;
use serde_json::Result;
use std::error::Error;
use tokio;

// 定义一个简单的数据结构来表示待验证的数据
#[derive(Deserialize, Debug)]
struct Data {
    name: String,
    age: u32,
}

// 创建一个函数来验证数据
async fn validate_data(data_str: &str) -> Result<(), Box<dyn Error>> {
    let data: Data = serde_json::from_str(data_str)?;
    
    // 验证名称是否不为空
    if data.name.is_empty() {
        return Err("Name cannot be empty".into());
    }
    
    // 验证年龄是否在合理的范围内
    if data.age < 0 || data.age > 120 {
        return Err("Age must be between 0 and 120".into());
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    // 测试数据，这部分代码应该被替换为实际的数据源
    let test_data = r#"{
        "name": "John Doe",
        "age": 30
    }"#;
    
    // 调用验证函数并处理可能的错误
    match validate_data(test_data).await {
        Ok(_) => println!("Data is valid"),
        Err(e) => println!("Error validating data: {}", e),
    }
}
