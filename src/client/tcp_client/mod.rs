use std::net::IpAddr;

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process;

pub struct TcpClient {
  ip: IpAddr,
  port: u16,
  stream: TcpStream
}

impl TcpClient {
  pub async fn new(ip: IpAddr, port: u16) -> Result<Self, Box<dyn std::error::Error>>{
    let stream = match TcpStream::connect((ip, port)).await {
      Ok(stream) => stream,
      Err(e) => {
        eprintln!("Failed to connect to {}:{}", ip, port);
        return Err(Box::new(e));
      }
    };
    Ok(TcpClient{ip, port, stream})
  }

  pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    for i in 1..10 {
      let message = format!("Hi from {}", process::id()).into_bytes();
      let _ = self.stream.write_all(&message).await;

    }
    Ok(())
  }
}