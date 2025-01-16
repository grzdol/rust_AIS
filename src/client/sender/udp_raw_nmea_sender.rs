use tokio::net::UdpSocket;

use crate::utils::{split_message_on_TIMESTAMP, string_to_msg_type};

use super::Sender;

pub struct UdpRawNmeaSender {
    socket: UdpSocket,
}

impl Sender for UdpRawNmeaSender {
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
            let packet = string_to_msg_type(ais_message);
            let _ = self.socket.send(&packet);
        }
    }
}
