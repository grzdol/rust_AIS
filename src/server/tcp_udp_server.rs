use std::net::IpAddr;

use bytes::Bytes;
use tokio::{net::{TcpListener, TcpStream, UdpSocket}, sync::mpsc::{self, UnboundedReceiver, UnboundedSender}};
use receiver;
pub struct TcpUdpServer {
  udp_socket: UdpSocket,
  listener: TcpListener,
  opencpn_stream: TcpStream,
  tx: UnboundedSender<Bytes>,
  rx: UnboundedReceiver<Bytes>,
}

impl TcpUdpServer {
  pub async fn new(
    ip: IpAddr,
    port: u16,
    opencpn_server_address: &str,
) -> Result<Self, Box<dyn std::error::Error>> {
    let listener = match UdpSocket::bind((ip, port)).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to {}:{}", ip, port);
            return Err(Box::new(e));
        }
    };

    let opencpn_stream = match TcpStream::connect(opencpn_server_address).await {
        Ok(stream) => {
            println!("Connected to OpenCPN at {}", opencpn_server_address);
            stream
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    let (tx, rx) = mpsc::unbounded_channel::<Bytes>();

    Ok(TcpServer {
        listener,
        opencpn_stream,
        tx,
        rx,
    })
}
}