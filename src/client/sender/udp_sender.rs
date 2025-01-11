use tokio::net::UdpSocket;

use crate::utils::MsgType;

use super::Sender;

pub struct UdpSender {
    socket: UdpSocket,
}

impl Sender for UdpSender {
    fn send(&mut self, msg: MsgType) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let _ = self.socket.send(&msg).await;
        }
    }
}

impl UdpSender {
    pub async fn new(local_udp_addr: &str, server_udp_addr: &str) -> UdpSender {
        let local_udp_sock = UdpSocket::bind(local_udp_addr).await.unwrap();
        local_udp_sock.connect(server_udp_addr).await.unwrap();

        Self {
            socket: local_udp_sock,
        }
    }
}
