use bytes::Bytes;
use tokio::net::UdpSocket;


use super::Receiver;

pub struct UdpReceiver {
  sender: tokio::sync::mpsc::UnboundedSender<Bytes>,
  socket: UdpSocket
}

impl UdpReceiver {
    pub fn new(sender: tokio::sync::mpsc::UnboundedSender<Bytes>, socket: UdpSocket) -> Self {
        Self { sender, socket }
    }
}

impl Receiver<Bytes> for UdpReceiver {

    async fn recv(&mut self) -> Bytes {
      let mut buf = [0; 1024];
      let (len, _) = self.socket.recv_from(&mut buf).await.unwrap();
      // ToDo validate that we got proper ais message
      Bytes::copy_from_slice(&buf[..len])
    }

    async fn publish_data(&mut self, data: Bytes) -> () {
      self.sender.send(data);
    } 
}