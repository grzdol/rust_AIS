use std::net::IpAddr;
use std::thread::sleep;
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures::SinkExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::broadcast::{self, channel};
use tokio::{net::UdpSocket, sync::mpsc::UnboundedSender};
use tokio_util::codec::{FramedWrite, LinesCodec};

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

impl<T: BoatState + 'static> TcpUdpClient<T> {
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
            match boat_state_receiver.recv().await {
                Ok(mut msg) => {
                    msg.push('\n');
                    if let Err(e) = local_udp_sock.send(msg.as_bytes()).await {
                        eprintln!("Error sending message: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving boat state: {}", e);
                    break;
                }
            }
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
            match boat_state_receiver.recv().await {
                Ok(msg) => {
                    if let Err(e) = framed.send(msg).await {
                        eprintln!("Error sending message: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving boat state: {}", e);
                    break;
                }
            }
        }
    }

    pub async fn run(self) {
        let (tx, _rx): (broadcast::Sender<String>, broadcast::Receiver<String>) =
            broadcast::channel(4096);
        let strong_channel = tx.subscribe();
        let weak_channel = tx.subscribe();
        let strong_sender = tokio::spawn(async move {
            let _ = TcpUdpClient::<T>::create_and_run_strong_sender(
                self.tcp_addr,
                self.tcp_port,
                strong_channel,
            )
            .await;
        });
        let weak_sender = tokio::spawn(async move {
            let _ = TcpUdpClient::<T>::create_and_run_weak_sender(
                self.local_udp_addr,
                self.local_udp_port,
                self.server_udp_addr,
                self.server_udp_port,
                weak_channel,
            )
            .await;
        });
        let boat_state_publisher = tokio::spawn(async move {
            let _ = TcpUdpClient::<T>::boat_state_handler(self.boat_state, tx).await;
        });

        let _ = tokio::join!(strong_sender, weak_sender, boat_state_publisher);
    }
}
