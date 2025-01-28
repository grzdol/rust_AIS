use tokio::net::UdpSocket;

use crate::utils::{MsgType, MSGTYPESIZE};

use super::{Receiver, ReceiverT};

pub struct UdpReceiver {
    finish_accepting: bool,
    socket: Option<UdpSocket>,
}

impl UdpReceiver {
    pub async fn new(udp_receiver_addr: &str) -> Self {
        let udp_sock = match UdpSocket::bind(udp_receiver_addr).await {
            Ok(socket) => Some(socket),
            Err(_) => None,
        };
        Self {
            finish_accepting: false,
            socket: udp_sock,
        }
    }
}

impl Receiver<UdpSocket> for UdpReceiver {
    async fn accept_client(&mut self) -> UdpSocket {
        self.finish_accepting = true;
        self.socket.take().unwrap()
    }

    async fn recv(socket: &mut UdpSocket) -> MsgType {
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

    fn finish_accepting(&self) -> bool {
        self.finish_accepting
    }
}

impl ReceiverT for UdpReceiver {
    type AcceptArgs = UdpSocket;

    type Receiver = UdpReceiver;
}
