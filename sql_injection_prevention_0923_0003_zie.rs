use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use std::str;

/// 异步处理客户端请求
async fn handle_client(mut socket: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];
    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) if n == 0 => return Ok(()),
            Ok(n) => n,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "failed to read from socket")),
        };

        // 简单的示例：假设客户端发送的请求以“SELECT * FROM users WHERE name = ”开头
        // 并跟随用户输入的用户名
        if let Ok(request) = str::from_utf8(&buf[..n]) {
            let user_input = request.trim_start_matches("SELECT * FROM users WHERE name = ");

            // 这里使用参数化查询来防止SQL注入
            let query = "SELECT * FROM users WHERE name = $1";
            let pool = sqlx::postgres::PgPoolOptions::new()
                .connect("host=localhost user=postgres password=yourpassword")
                .await?;

            let result = pool.fetch_one(query, &[user_input]).await;

            match result {
                Ok(row) => {
                    let response = format!("User found: {}", row.get("name"));
                    socket.write_all(response.as_bytes()).await?;
                },
                Err(e) => {
                    let response = format!("An error occurred: {}", e);
                    socket.write_all(response.as_bytes()).await?;
                },
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}
