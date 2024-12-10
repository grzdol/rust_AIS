use super::Sender;
use crate::{boat_state::BoatState, utils::{self, AISData}};
use bytes::Bytes;
use tokio::net::UdpSocket;
use std::net::SocketAddr;

pub struct UdpWeakSender<T> where T: BoatState {
  socket: UdpSocket,
  server_socket_addr: SocketAddr,
  sender: tokio::sync::mpsc::UnboundedSender<Bytes>, //sends data to strong sender
  boat_state: T

}

impl<T> Sender<AISData> for UdpWeakSender<T> where T: BoatState
{
    async fn send(&mut self, data: AISData) {
        let encoded_ais_data = utils::encode_ais_data(data).await.unwrap();
        self.sender.send(Bytes::from(encoded_ais_data.clone()));
        self.socket.send_to(encoded_ais_data.as_bytes(), self.server_socket_addr).await;
    }

    async fn get_data(&mut self) -> AISData{
      self.boat_state.get_ais_data()
    }
}