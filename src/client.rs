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
use tokio::sync::mpsc;

use crate::broadcaster::Broadcaster;
use crate::{
    boat_state::BoatState,
    utils::{self, build_timestamped_ais_message, encode_ais_data, AISResponse, MsgType},
    broadcaster::broadcaster_mockup::BroadcasterMockup
};

pub mod tcp_client;

pub struct TcpUdpClient<T: BoatState> {
    tcp_addr: String,
    local_udp_addr: String,
    server_udp_addr: String,
    boat_state: T,
    send_channel: broadcast::Sender<MsgType>,
    recv_channel: broadcast::Receiver<MsgType>
}

impl<T: BoatState + 'static> TcpUdpClient<T> {
    pub fn new(tcp_addr: &str, local_udp_addr: &str, server_udp_addr: &str, boat_state: T, send_channel: broadcast::Sender<MsgType>,recv_channel: broadcast::Receiver<MsgType>) -> Self {
        TcpUdpClient {
            tcp_addr: tcp_addr.to_string(),
            local_udp_addr: local_udp_addr.to_string(),
            server_udp_addr: server_udp_addr.to_string(),
            boat_state,
            send_channel,
            recv_channel
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
            let _ = boat_state_channel.send(msg);
        }
    }

    async fn create_and_run_weak_sender(
        local_udp_addr: &str,
        server_udp_addr: &str,
        mut boat_state_receiver: broadcast::Receiver<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let local_udp_sock = UdpSocket::bind(local_udp_addr).await?;
        local_udp_sock.connect(server_udp_addr).await?;
        loop {
            match boat_state_receiver.recv().await {
                Ok(msg) => {
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
        Ok(())
    }

    async fn create_and_run_strong_sender(
        tcp_addr: &str,
        mut boat_state_receiver: broadcast::Receiver<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let stream = match tokio::net::TcpStream::connect(tcp_addr).await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to connect to {}: {}", tcp_addr, e);
                return Err(Box::new(e));
            }
        };
        let mut framed =
            tokio_util::codec::FramedWrite::new(stream, tokio_util::codec::LinesCodec::new());
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
        Ok(())
    }

    pub async fn run(self) {
        let (tx, _rx): (broadcast::Sender<String>, broadcast::Receiver<String>) =
            broadcast::channel(4096);
        let strong_channel = tx.subscribe();
        let weak_channel = tx.subscribe();
        let strong_sender = tokio::spawn(async move {
            let _ = TcpUdpClient::<T>::create_and_run_strong_sender(&self.tcp_addr, strong_channel)
                .await;
        });
        let weak_sender = tokio::spawn(async move {
            let _ = TcpUdpClient::<T>::create_and_run_weak_sender(
                &self.local_udp_addr,
                &self.server_udp_addr,
                weak_channel,
            )
            .await;
        });
        let boat_state_publisher = tokio::spawn(async move {
            let _ = TcpUdpClient::<T>::boat_state_handler(self.boat_state, tx).await;
        });

        let broadcaster_handle = tokio::spawn({
            async move {
                let recv1 = self.send_channel.subscribe();
                let (send_channel, recv_channel) = mpsc::unbounded_channel();
                BroadcasterMockup::<MsgType>::run(recv1, self.send_channel, (), recv_channel, send_channel).await;
            }
        });
 

        let _ = tokio::join!(strong_sender, weak_sender, boat_state_publisher, broadcaster_handle);
    }
}
