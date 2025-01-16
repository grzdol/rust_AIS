use nmea::sentences::utils;
use tokio::net::UdpSocket;

use crate::utils::{MsgType, MSGTYPESIZE};

use super::Receiver;

pub struct UdpReceiver {
    finish_accepting: bool,
    socket: Option<UdpSocket>,
}

impl Receiver<UdpSocket> for UdpReceiver {
    fn accept_client(&mut self) -> impl std::future::Future<Output = (UdpSocket)> + Send {
        async {
            self.finish_accepting = true;
            self.socket.take().unwrap()
        }
    }

    fn recv(socket: UdpSocket) -> impl std::future::Future<Output = MsgType> + Send {
        async move {
            let mut buf = [0u8; MSGTYPESIZE + 1];
            let (len, _) = match socket.recv_from(&mut buf).await {
                Ok(result) => result,
                Err(e) => {
                    panic!("Error receiving from UDP socket: {}", e);
                }
            };

            let mut msg = [0u8; MSGTYPESIZE];
            let copy_len = MSGTYPESIZE.min(len);
            msg[..copy_len].copy_from_slice(&buf[..copy_len]);
            msg
        }
    }

    fn finish_accepting(&self) -> bool {
        self.finish_accepting
    }
}
