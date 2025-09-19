use tokio::sync::Mutex;
use tokio::sync::mpsc;
use std::collections::HashMap;
use std::sync::Arc;

/// 订单项
struct OrderItem {
    /// 商品ID
    product_id: String,
    /// 数量
    quantity: u32,
}

/// 订单
struct Order {
    /// 订单ID
    order_id: String,
    /// 订单项列表
    items: Vec<OrderItem>,
    /// 订单状态
    status: OrderStatus,
}

/// 订单状态
enum OrderStatus {
    New,
    Processing,
    Completed,
    Cancelled,
}

/// 订单处理程序
#[derive(Clone)]
struct OrderProcessor {
    /// 订单存储
    orders: Arc<Mutex<HashMap<String, Order>>>,
}

impl OrderProcessor {
    /// 创建一个新的订单处理程序
    pub fn new() -> Self {
        OrderProcessor {
            orders: Arc::new(Mutex::new(HashMap::new())),
        },
    }

    /// 创建一个新订单
    async fn create_order(&self, order_id: String, items: Vec<OrderItem>) {
        let mut orders = self.orders.lock().await;
        let order = Order {
            order_id,
            items,
            status: OrderStatus::New,
        };
        orders.insert(order_id, order);
    }

    /// 处理订单
    async fn process_order(&self, order_id: String) -> Result<(), String> {
        let mut orders = self.orders.lock().await;
        if let Some(order) = orders.get_mut(&order_id) {
            if order.status == OrderStatus::New {
                order.status = OrderStatus::Processing;
                // 处理订单逻辑
                // ...
                order.status = OrderStatus::Completed;
                Ok(())
            } else {
                Err("Order is not in a valid state for processing".to_string())
            }
        } else {
            Err("Order not found".to_string())
        }
    }

    /// 取消订单
    async fn cancel_order(&self, order_id: String) -> Result<(), String> {
        let mut orders = self.orders.lock().await;
        if let Some(order) = orders.get_mut(&order_id) {
            if order.status == OrderStatus::New || order.status == OrderStatus::Processing {
                order.status = OrderStatus::Cancelled;
                Ok(())
            } else {
                Err("Order cannot be cancelled in the current state".to_string())
            }
        } else {
            Err("Order not found".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    let processor = OrderProcessor::new();

    // 创建订单
    processor.create_order("order1".to_string(), vec![OrderItem { product_id: "product1".to_string(), quantity: 2 }]).await;

    // 处理订单
    if let Err(e) = processor.process_order("order1".to_string()).await {
        eprintln!("Failed to process order: {}", e);
    }

    // 取消订单
    if let Err(e) = processor.cancel_order("order1".to_string()).await {
        eprintln!("Failed to cancel order: {}", e);
    }
}
