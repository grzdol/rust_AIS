use std::net::IpAddr;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use tokio::task;

pub struct TcpServer {
  listener: TcpListener
}

impl TcpServer {
  pub async fn new(ip: IpAddr, port: u16) -> Result<Self, Box<dyn std::error::Error>> {
    let listener = match TcpListener::bind((ip, port)).await {
      Ok(listener) => listener,
      Err(e) => {
          eprintln!("Failed to bind to {}:{}", ip, port);
          return Err(Box::new(e));
      }
    };
    Ok(TcpServer {listener })
  }

  async fn handle_client( mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];
    loop {
        let n: usize = socket.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        // Convert the buffer to a string slice and print it
        if let Ok(message) = std::str::from_utf8(&buf[..n]) {
            println!("{}", message);
        } else {
            println!("Received non-UTF8 data: {:?}", &buf[..n]);
        }
    }
    Ok(())
  }

  pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    loop {
      // Accept an incoming connection
      let (socket, addr) = self.listener.accept().await?;
      println!("New connection from: {}", addr);
      task::spawn(async move {
          if let Err(e) = TcpServer::handle_client(socket).await {
              eprintln!("Error handling client {}: {}", addr, e);
          }
      });
  }
  }
}