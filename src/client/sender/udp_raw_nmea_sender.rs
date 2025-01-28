use tokio::net::UdpSocket;

use crate::utils::{split_message_on_timestamp, string_to_msg_type};

use super::Sender;

pub struct UdpRawNmeaSender {
    socket: UdpSocket,
}

impl UdpRawNmeaSender {
    pub async fn new(server_addr: &str) -> Self {
        let socket = UdpSocket::bind(server_addr)
            .await
            .expect("Failed to bind socket");
        Self { socket }
    }
}

impl Sender for UdpRawNmeaSender {
    async fn send(&mut self, msg: crate::utils::MsgType) {
        let (ais_message, _timestamp) =
            match split_message_on_timestamp(String::from_utf8(msg.to_vec()).unwrap()) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error splitting message on TIMESTAMP: {}", e);
                    return;
                }
            };
        let packet = string_to_msg_type(ais_message);
        let _ = self.socket.send(&packet).await;
    }
}
