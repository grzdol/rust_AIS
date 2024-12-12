use std::net::IpAddr;

use std::net::SocketAddr;
use bytes::Bytes;
use tokio::net::{TcpStream, UdpSocket};

use super::Client;
use crate::boat_state::BoatState;
use crate::client::sender::udp_weak_sender::UdpWeakSender;
use crate::client::sender::tcp_strong_sender::TcpStrongSender;

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

impl<T> Client<T, TcpStrongSender, UdpWeakSender<T>> for TcpUdpClient where  T: BoatState {
    fn get_weak_sender(&mut self) -> UdpWeakSender<T> {
        todo!()
    }

    fn get_strong_sender(&mut self) -> TcpStrongSender {
        todo!()
    }
}