use std::net::IpAddr;

use bytes::Bytes;
use futures::io::Lines;
use futures::SinkExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::task;
use tokio_stream::StreamExt;
// use::codec::{Framed, LinesCodec};
// use tokio_utils::codec::{LinesCodec, Framed};
use tokio_util::codec::{Framed, FramedRead, FramedWrite, LinesCodec};

use crate::client::sender::tcp_raw_nmea_sender::TcpRawNmeaSender;
use crate::client::sender::tcp_sender::TcpSender;
use crate::client::sender::udp_raw_nmea_sender::UdpRawNmeaSender;
use crate::utils::{get_next_framed_ais_message, split_message_on_TIMESTAMP};

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
