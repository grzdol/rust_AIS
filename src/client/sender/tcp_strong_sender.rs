use super::Sender;
use crate::{boat_state::BoatState, utils::{self, AISData}};
use bytes::Bytes;
use tokio::{io::AsyncWriteExt, net::{unix::SocketAddr, TcpStream}};


pub struct TcpStrongSender where {
  stream: TcpStream,
  server_socket_addr: SocketAddr,
  receiver: tokio::sync::mpsc::UnboundedReceiver<AISData>, //sends data to strong sender
}

impl Sender<AISData> for TcpStrongSender 
{
    async fn send(&mut self, data: AISData) {
        let encoded_ais_data = utils::encode_ais_data(data).await.unwrap();
        self.stream.write_all(encoded_ais_data.as_bytes()).await;
    }

    async fn get_data(&mut self) -> AISData{
      self.receiver.recv().await.unwrap()
    }
}