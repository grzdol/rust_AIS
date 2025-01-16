use crate::{
    server::Receiver,
    utils::{string_to_msg_type, MsgType},
};
use futures::{io::Lines, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

pub struct TcpReceiver {
    listener: TcpListener,
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
        mut framed: FramedRead<TcpStream, LinesCodec>,
    ) -> impl std::future::Future<Output = MsgType> + Send {
        async move {
            match framed.next().await {
                Some(Ok(line)) => string_to_msg_type(line),
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
