use tokio::sync::Mutex;
use std::sync::Arc;
use std::error::Error;
use std::fmt;
use async_trait::async_trait;

// 定义支付错误
#[derive(Debug)]
struct PaymentError {
    message: String,
}

impl PaymentError {
    fn new(message: &str) -> PaymentError {
        PaymentError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PaymentError {}

// 定义支付接口
#[async_trait]
trait PaymentProcessor {
    async fn process_payment(&self, amount: f64) -> Result<(), Box<dyn Error>>;
}

// 实现一个具体的支付处理器
struct StripePaymentProcessor;

#[async_trait]
impl PaymentProcessor for StripePaymentProcessor {
    async fn process_payment(&self, amount: f64) -> Result<(), Box<dyn Error>> {
        // 这里模拟支付处理逻辑
        if amount <= 0.0 {
            return Err(Box::new(PaymentError::new("Amount must be greater than 0")));
        }

        // 模拟支付成功
        println!("Payment of {} processed successfully", amount);
        Ok(())
    }
}

// 支付服务，包含支付处理器
struct PaymentService {
    processor: Arc<Mutex<dyn PaymentProcessor>>,
}

impl PaymentService {
    // 创建支付服务实例
    fn new(processor: Arc<Mutex<dyn PaymentProcessor>>) -> Self {
        PaymentService { processor }
    }

    // 处理支付
    async fn handle_payment(&self, amount: f64) -> Result<(), Box<dyn Error>> {
        let processor = self.processor.lock().await;
        processor.process_payment(amount).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 创建支付处理器实例
    let processor = Arc::new(Mutex::new(StripePaymentProcessor));

    // 创建支付服务实例
    let service = PaymentService::new(processor);

    // 处理支付
    service.handle_payment(100.0).await?;
    Ok(())
}
