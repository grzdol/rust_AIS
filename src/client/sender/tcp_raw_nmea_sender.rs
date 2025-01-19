use futures::{io::Lines, SinkExt};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedWrite, LinesCodec};

use crate::utils::{split_message_on_TIMESTAMP, string_to_msg_type};

use super::Sender;

pub struct TcpRawNmeaSender {
  framed: FramedWrite<TcpStream, LinesCodec>,
}

impl TcpRawNmeaSender {
    pub async fn new(server_address: &str) -> Self {
        let stream = TcpStream::connect(server_address).await.unwrap();
        let framed = FramedWrite::new(stream, LinesCodec::new());
        Self { framed }
    }
}

impl Sender for TcpRawNmeaSender {
    fn send(&mut self, msg: crate::utils::MsgType) -> impl std::future::Future<Output = ()> + Send {
      async move {
        let (ais_message, timestamp) =
            match split_message_on_TIMESTAMP(String::from_utf8(msg.to_vec()).unwrap()) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error splitting message on TIMESTAMP: {}", e);
                    return;
                }
            };

        let _ = self.framed.send(ais_message).await ;
    }
    }
}