use std::net::IpAddr;
use std::thread::sleep;
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures::SinkExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::broadcast::{self, channel};
use tokio::sync::mpsc;
use tokio::{net::UdpSocket, sync::mpsc::UnboundedSender};
use tokio_util::codec::{FramedWrite, LinesCodec};

use crate::broadcaster::{self, Broadcaster};
use crate::utils::string_to_msg_type;
use crate::{
    boat_state::BoatState,
    broadcaster::broadcaster_mockup::BroadcasterMockup,
    broadcaster::BroadcasterParams,
    utils::{self, build_timestamped_ais_message, encode_ais_data, AISResponse, MsgType},
};
use std::hash::Hash;
use std::marker::PhantomData;

use super::sender::tcp_sender::TcpSender;
use super::sender::udp_sender::UdpSender;
use super::Client;

pub struct TcpUdpClient<BP: BroadcasterParams> {
    broadcaster: Option<BP::B>,
}

impl<T: BoatState, BP: BroadcasterParams> Client<T, UdpSender, TcpSender, BP> for TcpUdpClient<BP> {
    fn get_broadcaster(
        &mut self,
        broadcaster_recv_channel: mpsc::UnboundedReceiver<MsgType>,
    ) -> BP::B{
        let broadcaster = self.broadcaster.take().expect("No broadcaster. Panic");
        broadcaster.set_recv_channel(broadcaster_recv_channel);
        
    }
}
// pub struct TcpUdpClient<T: BoatState> {
//     tcp_addr: String,
//     local_udp_addr: String,
//     server_udp_addr: String,
//     boat_state: T,
//     send_channel: broadcast::Sender<MsgType>,
//     recv_channel: broadcast::Receiver<MsgType>,
// }

// impl<T: BoatState + 'static> TcpUdpClient<T> {
//     pub fn new(
//         tcp_addr: &str,
//         local_udp_addr: &str,
//         server_udp_addr: &str,
//         boat_state: T,
//         send_channel: broadcast::Sender<MsgType>,
//         recv_channel: broadcast::Receiver<MsgType>,
//     ) -> Self {
//         TcpUdpClient {
//             tcp_addr: tcp_addr.to_string(),
//             local_udp_addr: local_udp_addr.to_string(),
//             server_udp_addr: server_udp_addr.to_string(),
//             boat_state,
//             send_channel,
//             recv_channel,
//         }
//     }

//     async fn boat_state_handler(boat_state: T, boat_state_channel: broadcast::Sender<String>) {
//         loop {
//             //ToDo this should be async call on boat state
//             sleep(Duration::new(1, 0));
//             let data = boat_state.get_ais_data();
//             let encoded_data = encode_ais_data(data).await.unwrap();
//             let timestamp = chrono::offset::Utc::now().to_rfc3339();
//             let response = AISResponse {
//                 timestamp,
//                 ais_message: encoded_data,
//             };
//             let msg = build_timestamped_ais_message(response);
//             let _ = boat_state_channel.send(msg);
//         }
//     }

//     async fn run_weak_sender(
//         local_udp_addr: &str,
//         server_udp_addr: &str,
//         mut boat_state_receiver: broadcast::Receiver<String>,
//         broadcaster_send_channel: UnboundedSender<MsgType>,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         loop {
//             match boat_state_receiver.recv().await {
//                 Ok(msg) => {
//                     let msg_len = msg.len();
//                     let msg_converted = string_to_msg_type(msg);
//                     if let Err(e) = local_udp_sock.send(&msg_converted[..msg_len]).await {
//                         eprintln!("Error sending message: {}", e);
//                         break;
//                     }
//                     broadcaster_send_channel.send(msg_converted);
//                 }
//                 Err(e) => {
//                     eprintln!("Error receiving boat state: {}", e);
//                     break;
//                 }
//             }
//         }
//         Ok(())
//     }

//     async fn run_strong_sender(
//         tcp_addr: &str,
//         mut boat_state_receiver: broadcast::Receiver<String>,
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         let stream = match tokio::net::TcpStream::connect(tcp_addr).await {
//             Ok(stream) => stream,
//             Err(e) => {
//                 eprintln!("Failed to connect to {}: {}", tcp_addr, e);
//                 return Err(Box::new(e));
//             }
//         };
//         let mut framed =
//             tokio_util::codec::FramedWrite::new(stream, tokio_util::codec::LinesCodec::new());
//         loop {
//             match boat_state_receiver.recv().await {
//                 Ok(msg) => {
//                     if let Err(e) = framed.send(msg).await {
//                         eprintln!("Error sending message: {}", e);
//                         break;
//                     }
//                 }
//                 Err(e) => {
//                     eprintln!("Error receiving boat state: {}", e);
//                     break;
//                 }
//             }
//         }
//         Ok(())
//     }

//     pub async fn run(self) {
//         let (tx, _rx): (broadcast::Sender<String>, broadcast::Receiver<String>) =
//             broadcast::channel(4096);
//         let strong_channel = tx.subscribe();
//         let weak_channel = tx.subscribe();
//         let strong_sender = tokio::spawn(async move {
//             let _ = TcpUdpClient::<T>::run_strong_sender(&self.tcp_addr, strong_channel).await;
//         });

//         //Those are channels through which workers in broadcaster talk.
//         //We also send with send channel msgs to broadcasters sender
//         let (send_channel, recv_channel) = mpsc::unbounded_channel();
//         let weak = send_channel.clone();

//         let weak_sender = tokio::spawn(async move {
//             let _ = TcpUdpClient::<T>::run_weak_sender(
//                 &self.local_udp_addr,
//                 &self.server_udp_addr,
//                 weak_channel,
//                 weak,
//             )
//             .await;
//         });
//         let boat_state_publisher = tokio::spawn(async move {
//             let _ = TcpUdpClient::<T>::boat_state_handler(self.boat_state, tx).await;
//         });

//         let broadcaster_handle = tokio::spawn({
//             async move {
//                 let recv1 = self.send_channel.subscribe();

//                 BroadcasterMockup::run(recv1, self.send_channel, (), recv_channel, send_channel)
//                     .await;
//             }
//         });

//         let _ = tokio::join!(
//             strong_sender,
//             weak_sender,
//             boat_state_publisher,
//             broadcaster_handle
//         );
//     }
// }
