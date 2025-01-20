use tokio::net::UdpSocket;

use crate::utils::{MsgType, MSGTYPESIZE};

use super::BoatState;

pub struct BoatStateUdp{
  socket: UdpSocket
}

impl BoatStateUdp {
    pub async fn new(local_addr: &str, client_addr: &str) -> Self {
        let socket= UdpSocket::bind(local_addr).await.unwrap();
        let _ = socket.connect(client_addr).await;
        Self { socket }
    }
}

impl BoatState for BoatStateUdp {
    fn get_ais_data(&self) -> impl std::future::Future<Output = crate::utils::MsgType> + std::marker::Send {
      async move {
        let mut buf: MsgType = [0u8; MSGTYPESIZE];
        let _ = self.socket.recv(&mut buf).await;
        buf
      }
    }
}

