// use::codec::{Framed, LinesCodec};
// use tokio_utils::codec::{LinesCodec, Framed};

use crate::client::sender::tcp_raw_nmea_sender::TcpRawNmeaSender;

use super::receiver::tcp_receiver::TcpReceiver;
use super::receiver::udp_receiver::UdpReceiver;
use super::Server;

pub struct TcpUdpServer<'a> {
    listener_addr: &'a str,
    udp_receiver_addr: &'a str,
    real_time_server_address: &'a str,
    history_server_address: &'a str,
}

impl<'a> TcpUdpServer<'a> {
    pub fn new(
        listener_addr: &'a str,
        udp_receiver_addr: &'a str,
        real_time_server_address: &'a str,
        history_server_address: &'a str,
    ) -> Self {
        TcpUdpServer {
            listener_addr,
            udp_receiver_addr,
            real_time_server_address,
            history_server_address,
        }
    }
}
impl<'a> Server<UdpReceiver, TcpReceiver, TcpRawNmeaSender, TcpRawNmeaSender> for TcpUdpServer<'a> {
    async fn get_strong_receiver(&mut self) -> TcpReceiver {
        TcpReceiver::new(self.listener_addr).await
    }

    async fn get_weak_receiver(&mut self) -> UdpReceiver {
        UdpReceiver::new(self.udp_receiver_addr).await
    }

    async fn get_strong_publisher(&mut self) -> TcpRawNmeaSender {
        TcpRawNmeaSender::new(self.history_server_address).await
    }

    async fn get_weak_publisher(&mut self) -> TcpRawNmeaSender {
        TcpRawNmeaSender::new(self.real_time_server_address).await
    }
}
