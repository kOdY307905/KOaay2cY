use tokio;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;
use std::time::SystemTime;
use std::error::Error;
use serde_json::json;
use std::collections::HashMap;

// 定义测试用例结果结构体
struct TestCaseResult {
    name: String,
    status: String,
    duration: u64,
    output: String,
}

// 定义测试报告生成器
struct TestReportGenerator {
    output_path: PathBuf,
    test_results: HashMap<String, TestCaseResult>,
}

impl TestReportGenerator {
    // 创建一个新的测试报告生成器
    fn new(output_path: PathBuf) -> Self {
        TestReportGenerator {
            output_path,
            test_results: HashMap::new(),
        }
    }

    // 添加测试用例结果
    fn add_test_result(&mut self, name: String, status: String, duration: u64, output: String) {
        let result = TestCaseResult {
            name,
            status,
            duration,
            output,
        };
        self.test_results.insert(name, result);
    }

    // 生成测试报告
    async fn generate_report(&self) -> Result<(), Box<dyn Error>> {
        let report = self.generate_report_content().await?;
        self.write_report_to_file(&report).await?;
        Ok(())
    }

    // 生成报告内容
    async fn generate_report_content(&self) -> Result<String, Box<dyn Error>> {
        let mut report = String::new();
        report.push_str("Test Report
");
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        report.push_str(&format!("Generated at: {}
", current_time));
        report.push_str("Test Cases:
");
        for (name, result) in &self.test_results {
            report.push_str(&format!("{}
", json!({
                "name": name,
                "status": result.status,
                "duration": result.duration,
                "output": result.output,
            }).to_string()));
        }
        Ok(report)
    }

    // 将报告写入文件
    async fn write_report_to_file(&self, report: &str) -> Result<(), Box<dyn Error>> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.output_path)
            .await?;
        file.write_all(report.as_bytes()).await?;
        file.flush().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut generator = TestReportGenerator::new(PathBuf::from("test_report.txt"));
    generator.add_test_result("test_case_1".to_string(), "passed".to_string(), 100, "Test output".to_string());
    generator.add_test_result("test_case_2".to_string(), "failed".to_string(), 200, "Test output".to_string());
    generator.generate_report().await?;
    Ok(())
}
