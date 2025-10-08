use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use tokio;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};
use std::io::Error;

#[derive(Serialize, Deserialize, Debug)]
struct JsonTransformRequest {
    json_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonTransformResponse {
    transformed_json: String,
}

async fn handle_client(mut socket: tokio::net::TcpStream) -> io::Result<()> {
    let mut lines = FramedRead::new(socket, LinesCodec::new());
    let mut line = String::new();
    while lines.read_line(&mut line).await? != 0 {
        let request: JsonTransformRequest = serde_json::from_str(&line).map_err(|e|
            io::Error::new(io::ErrorKind::InvalidData, e))?;
        let response = JsonTransformResponse {
            transformed_json: request.json_data.clone(),
        };
        let response_str = serde_json::to_string(&response).map_err(|e|
            io::Error::new(io::ErrorKind::InvalidData, e))?;
        socket.write_all(response_str.as_bytes()).await?;
        socket.write_all(b"
").await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(handle_client(socket));
    }
}
