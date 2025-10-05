use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use async_trait::async_trait;
use anyhow::Error;

/// Represents a fraud detection service
#[derive(Debug, Clone)]
pub struct FraudDetectionService {
    /// A map to store user data
    user_data: Arc<Mutex<HashMap<String, UserData>>>,
}

/// Represents user data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    /// User's name
    name: String,
    /// User's age
    age: u32,
    /// User's history of transactions
    transactions: Vec<Transaction>,
}

/// Represents a transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    /// Transaction amount
    amount: f64,
    /// Transaction date
    date: String,
}

#[async_trait]
pub trait FraudDetection {
    async fn detect_fraud(&self, user_id: &str) -> Result<bool, Error>;
}

impl FraudDetectionService {
    /// Creates a new instance of FraudDetectionService
    pub fn new() -> Self {
        FraudDetectionService {
            user_data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds a user to the service
    pub async fn add_user(&self, user: UserData) {
        let mut users = self.user_data.lock().await;
        users.insert(user.name.clone(), user);
    }
}

#[async_trait]
impl FraudDetection for FraudDetectionService {
    /// Detects fraud by analyzing user transactions
    async fn detect_fraud(&self, user_id: &str) -> Result<bool, Error> {
        let users = self.user_data.lock().await;
        if let Some(user) = users.get(user_id) {
            // Simple fraud detection logic: If a user has more than 10 transactions
            // or transactions with an amount greater than $1000, it's considered fraudulent.
            if user.transactions.len() > 10 || user.transactions.iter().any(|t| t.amount > 1000.0) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(Error::msg("User not found"))
        }
    }
}

#[tokio::main]
async fn main() {
    let service = FraudDetectionService::new();

    // Example user data
    let user = UserData {
        name: "John Doe".to_string(),
        age: 30,
        transactions: vec![
            Transaction { amount: 100.0, date: "2023-01-01".to_string() },
            Transaction { amount: 200.0, date: "2023-01-02".to_string() },
            // Add more transactions as needed
        ],
    };

    // Add user to the service
    service.add_user(user).await;

    // Detect fraud for the user
    match service.detect_fraud(&user.name).await {
        Ok(is_fraud) => println!("Fraud detected: {}", is_fraud),
        Err(e) => println!("Error detecting fraud: {}", e),
    }
}