use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        // Accept an incoming connection
        let (socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        // Spawn a new task to handle the connection
        task::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0; 1024];

    loop {
        // Read data from the socket
        let n = socket.read(&mut buf).await?;

        // If no data was read, the client disconnected
        if n == 0 {
            break;
        }

        // Echo the data back to the client
        socket.write_all(&buf[..n]).await?;
    }

    Ok(())
}
