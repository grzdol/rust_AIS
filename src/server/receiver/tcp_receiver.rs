use bytes::Bytes;
use tokio::{io::AsyncReadExt, net::{TcpStream, UdpSocket}};


use super::Receiver;

pub struct TcpReceiver {
  sender: tokio::sync::mpsc::UnboundedSender<Bytes>,
  stream: TcpStream
}

impl Receiver<Bytes> for TcpReceiver {
    async fn recv(&mut self) -> Bytes {
      let mut buf = vec![0; 1024];
      let n = self.stream.read(&mut buf).await.expect("Failed to read from stream");
            if n == 0 {
                return Bytes::new();
            }
            Bytes::copy_from_slice(&buf[..n])
    }

    async fn publish_data(&mut self, data: Bytes) -> () {
      self.sender.send(data);
    } 
}

