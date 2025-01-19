use crate::{
    server::Receiver,
    utils::{string_to_msg_type, MsgType},
};
use futures::{io::Lines, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

use super::ReceiverT;

pub struct TcpReceiver {
    listener: TcpListener,
}

impl TcpReceiver {
    pub async fn new(listener_addr: &str) -> Self {
        let listener = TcpListener::bind(listener_addr).await.expect("Failed to bind to address");
        Self { listener }
    }
}

impl Receiver<FramedRead<TcpStream, LinesCodec>> for TcpReceiver {
    fn finish_accepting(&self) -> bool {
        false
    }

    fn accept_client(
        &mut self,
    ) -> impl std::future::Future<Output = (FramedRead<TcpStream, LinesCodec>)> + Send {
        async move {
            let (socket, _addr) = match self.listener.accept().await {
                Ok((socket, addr)) => (socket, addr),
                Err(e) => {
                    panic!("Failed to accept client: {}", e);
                }
            };
            FramedRead::new(socket, LinesCodec::new())
        }
    }

    fn recv(
        framed: &mut FramedRead<TcpStream, LinesCodec>,
    ) -> impl std::future::Future<Output = MsgType> + Send {
        async move {
            match framed.next().await {
                
                Some(Ok(line)) =>{
                    print!("{:?}", line);
                    string_to_msg_type(line)
                },
                Some(Err(e)) => {
                    panic!("Error receiving line: {}", e);
                }
                None => {
                    panic!("Stream ended.");
                }
            }
        }
    }
}

impl ReceiverT for TcpReceiver {
    type AcceptArgs = FramedRead<TcpStream, LinesCodec>;

    type Receiver = TcpReceiver;
}
