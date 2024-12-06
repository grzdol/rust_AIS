use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server!");

    let message = b"Hello, server!";
    
    // Send a message to the server
    stream.write_all(message).await?;
    println!("Sent: {:?}", String::from_utf8_lossy(message));

    // Read the response
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Received: {:?}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
