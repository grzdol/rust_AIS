mod single_thread_tcp_server;
mod client;
mod server;
use std::net::{IpAddr, Ipv4Addr};

use client::tcp_client;
use server::tcp_server;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let server = server::tcp_server::TcpServer::new(localhost_v4, 8080).await?;
    let mut handles = Vec::new();
    for i in 1..100 {
        let mut client = tcp_client::TcpClient::new(localhost_v4, 8080).await?;
        // tasks.push(tokio::spawn(async move {
        //     client.run().await
        // }))
        // client.run();
        let handle = tokio::spawn(async move {
            let _ = client.run().await;
        });
        handles.push(handle);
    }
    server.run().await?;
    
    Ok(())
    
}
