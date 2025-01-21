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

pub struct TcpUdpClient<BP: BroadcasterParams, T: BoatState> {
    broadcaster: Option<BP::B>,
    udp_sender: Option<UdpSender>,
    tcp_sender: Option<TcpSender>,
    boat_state: Option<T>,
}
impl<T: BoatState + 'static, BP: BroadcasterParams> Client<T, UdpSender, TcpSender, BP>
    for TcpUdpClient<BP, T>
{
    fn get_broadcaster(
        &mut self,
        broadcaster_recv_channel: mpsc::UnboundedReceiver<MsgType>,
        broadcaster_send_channel: mpsc::UnboundedSender<MsgType>,
    ) -> BP::B {
        self.broadcaster
            .as_mut()
            .expect("No broadcaster. Panic")
            .set_recv_channel(broadcaster_recv_channel, broadcaster_send_channel);
        self.broadcaster.take().expect("No broadcaster. Panic")
    }

    fn get_boat_state(&mut self) -> T {
        self.boat_state.take().expect("No boat state set")
    }

    async fn run(&mut self) {
        let weak_sender = self.udp_sender.take().expect("No udp sender");
        let strong_sender = self.tcp_sender.take().expect("No tcp sedner");
        self.run_impl(weak_sender, strong_sender).await;
    }
}

impl<BP: BroadcasterParams, T: BoatState> TcpUdpClient<BP, T> {
    pub fn new(
        broadcaster: BP::B,
        udp_sender: UdpSender,
        tcp_sender: TcpSender,
        boat_state: T,
    ) -> Self {
        Self {
            broadcaster: Some(broadcaster),
            udp_sender: Some(udp_sender),
            tcp_sender: Some(tcp_sender),
            boat_state: Some(boat_state),
        }
    }
}
