use std::env;
use tokio;
use tokio::net::UdpSocket;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
# NOTE: 重要实现细节
use tokio::runtime::Runtime;
use bytes::BytesMut;

/// 定义WiFi管理器结构体
struct WifiManager {
    socket: UdpSocket,
}

impl WifiManager {
    /// 创建新的WiFi管理器实例
    pub async fn new(interface: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(interface).await?;
        Ok(WifiManager { socket })
    }
# TODO: 优化性能

    /// 发送WiFi配置命令
    pub async fn send_command(&self, command: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.send(command).await?;
        Ok(())
    }

    /// 接收来自设备的响应
    pub async fn receive_response(&self, buffer: &mut [u8]) -> Result<usize, Box<dyn std::error::Error>> {
        self.socket.recv(buffer).await
    }
# 扩展功能模块
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <interface> <command>", args[0]);
        std::process::exit(1);
    }

    let interface = &args[1];
    let command = args[2].as_bytes();

    // 创建WiFi管理器
    let mut manager = WifiManager::new(interface).await?;

    // 发送命令
    manager.send_command(command).await?;

    // 接收响应
    let mut response_buffer = [0u8; 1024];
    let received = manager.receive_response(&mut response_buffer).await?;

    // 打印响应
    println!("Received: \x{:?}", &response_buffer[.received]);

    Ok(())
}
