use tokio::net::TcpStream;
use tokio_util::codec::{FramedWrite, LinesCodec};

use crate::utils::{msg_type_to_string, MsgType};

use super::Sender;
use futures::SinkExt;
pub struct TcpSender {
    framed_socket: FramedWrite<TcpStream, LinesCodec>,
}

impl Sender for TcpSender {
    async fn send(&mut self, msg: MsgType) {
        let _ = self.framed_socket.send(msg_type_to_string(msg)).await;
    }
}

impl TcpSender {
    pub async fn new(tcp_addr: &str) -> Self {
        let stream = match tokio::net::TcpStream::connect(tcp_addr).await {
            Ok(stream) => stream,
            Err(e) => {
                panic!("Failed to connect to {}: {}", tcp_addr, e);
            }
        };
        let framed_socket = FramedWrite::new(stream, LinesCodec::new());
        Self { framed_socket }
    }
}
