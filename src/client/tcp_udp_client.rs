use std::net::IpAddr;

use std::net::SocketAddr;
use tokio::net::{TcpStream, UdpSocket};

use super::Client;

pub struct TcpUdpClient {
    tcp_stream: TcpStream,
    udp_socket: UdpSocket,
    udp_addr: SocketAddr,
}

impl TcpUdpClient {
    pub async fn new(
        tcp_addr: &str,
        local_udp_addr: &str,
        server_udp_addr: &str,
    ) -> Result<Self, std::io::Error> {
        let local_udp_sock = UdpSocket::bind(local_udp_addr).await?;
        let tcp_stream = TcpStream::connect(tcp_addr).await?;
        let udp_addr = server_udp_addr.parse::<SocketAddr>().unwrap();
        Ok(TcpUdpClient {
            tcp_stream,
            udp_socket: local_udp_sock,
            udp_addr,
        })
    }
}
