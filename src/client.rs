use std::net::IpAddr;
use std::thread::sleep;
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio::sync::broadcast::{self, channel};
use tokio::{net::UdpSocket, sync::mpsc::UnboundedSender};
use tokio_util::codec::{FramedWrite, LinesCodec};
use tokio::io::AsyncWriteExt;

use crate::{
    boat_state::BoatState,
    utils::{self, build_timestamped_ais_message, encode_ais_data, AISResponse},
};

pub mod tcp_client;

pub struct TcpUdpClient<T: BoatState> {
    tcp_addr: IpAddr,
    tcp_port: u16,
    local_udp_addr: IpAddr,
    local_udp_port: u16,
    server_udp_addr: IpAddr,
    server_udp_port: u16,
    boat_state: T,
}

impl<T: BoatState> TcpUdpClient<T> {
    pub fn new(
        tcp_addr: IpAddr,
        tcp_port: u16,
        local_udp_addr: IpAddr,
        local_udp_port: u16,
        server_udp_addr: IpAddr,
        server_udp_port: u16,
        boat_state: T,
    ) -> Self {
        TcpUdpClient {
            tcp_addr,
            tcp_port,
            local_udp_addr,
            local_udp_port,
            server_udp_addr,
            server_udp_port,
            boat_state,
        }
    }

    async fn boat_state_handler(boat_state: T, boat_state_channel: broadcast::Sender<String>) {
        loop {
            //ToDo this should be async call on boat state
            sleep(Duration::new(1, 0));
            let data = boat_state.get_ais_data();
            let encoded_data = encode_ais_data(data).await.unwrap();
            let timestamp = chrono::offset::Utc::now().to_rfc3339();
            let response = AISResponse {
                timestamp,
                ais_message: encoded_data,
            };
            let msg = build_timestamped_ais_message(response);
            boat_state_channel.send(msg);
        }
    }

    async fn create_and_run_weak_sender(
        local_udp_addr: IpAddr,
        local_udp_port: u16,
        server_udp_addr: IpAddr,
        server_udp_port: u16,
        mut boat_state_receiver: broadcast::Receiver<String>,
    ) {
        let local_udp_sock = UdpSocket::bind((local_udp_addr, local_udp_port))
            .await
            .unwrap();
        let _ = local_udp_sock
            .connect((server_udp_addr, server_udp_port))
            .await;
        loop {
            let msg = boat_state_receiver.recv().await.unwrap();
            let _ = local_udp_sock.send(msg.as_bytes()).await;
        }
    }

    async fn create_and_run_strong_sender(
        tcp_addr: IpAddr,
        tcp_port: u16,
        mut boat_state_receiver: broadcast::Receiver<String>,
    ) {
      let stream = match TcpStream::connect((tcp_addr, tcp_port)).await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to {}:{}", tcp_addr, tcp_port);
            // return Err(Box::new(e));
            panic!()
        }
    };

      let mut framed = FramedWrite::new(stream, LinesCodec::new());
      loop {
        let msg = boat_state_receiver.recv().await.unwrap();
        let _ = framed.send(msg).await;
      }
        
    }
}
