use std::collections::HashSet;
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, mpsc};
use crate::broadcaster::Broadcaster;
use crate::client::sender::tcp_raw_nmea_sender::TcpRawNmeaSender;
use crate::client::sender::Sender;
use crate::server::receiver;
use crate::utils::{MsgType, MSGTYPESIZE};

pub struct UdpBroadcaster {
    sender_socket: Option<UdpSocket>,
    receiver_socket: Option<UdpSocket>,
    log_arg: Option<TcpRawNmeaSender>,
    local_recv_channel: Option<mpsc::UnboundedReceiver<MsgType>>,
    local_send_channel: Option<mpsc::UnboundedSender<MsgType>>,
    broadcast_address: &'static str
}

impl UdpBroadcaster {
    pub async fn new(
        local_recevier_address: &str,
        local_sender_address: &str,
        broadcast_address: &'static str,
        log_arg: TcpRawNmeaSender,
        local_recv_channel: mpsc::UnboundedReceiver<MsgType>,
        local_send_channel: mpsc::UnboundedSender<MsgType>,
    ) -> Self {
      let sender_socket = UdpSocket::bind(local_sender_address).await.unwrap();
      let _ = sender_socket.set_broadcast(true);
      let receiver_socket = UdpSocket::bind(local_recevier_address).await.unwrap();
      let _ = receiver_socket.set_broadcast(true);
        Self {
            sender_socket: Some(sender_socket),
            receiver_socket: Some(receiver_socket),
            log_arg: Some(log_arg),
            local_recv_channel: Some(local_recv_channel),
            local_send_channel: Some(local_send_channel),
            broadcast_address
        }
    }
}

impl Broadcaster<(UdpSocket, &'static str), UdpSocket, TcpRawNmeaSender> for UdpBroadcaster {
    fn broadcast(
        arg: &mut (UdpSocket, &'static str),
        msg: MsgType,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let _ = arg.0.send_to(&msg, arg.1).await;
        }
    }

    fn recv_from_broadcast(
        socket: &mut UdpSocket,
    ) -> impl std::future::Future<Output = MsgType> + Send {
      async move {
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
    }

    async fn log_received_from_broadcast(sender: &mut TcpRawNmeaSender, msg: MsgType) {
        sender.send(msg).await;
    }

    fn set_recv_channel(
        &mut self,
        recv_channel: mpsc::UnboundedReceiver<MsgType>,
        send_channel: mpsc::UnboundedSender<MsgType>,
    ) {
        self.local_recv_channel = Some(recv_channel);
        self.local_send_channel = Some(send_channel);
    }

    fn get_args(
        &mut self,
    ) -> (
        UdpSocket,
        (UdpSocket, &'static str),
        TcpRawNmeaSender,
        mpsc::UnboundedReceiver<MsgType>,
        mpsc::UnboundedSender<MsgType>,
    ) {
        (
            self.receiver_socket.take().expect("No receiver_socket in UdpBroadcaster"),
            (self.sender_socket.take().expect("No sender_socket in UdpBroadcaster"), self.broadcast_address),
            self.log_arg.take().expect("No log_arg in UdpBroadcaster"),
            self.local_recv_channel.take().expect("No local_recv_channel in UdpBroadcaster"),
            self.local_send_channel.take().expect("No local_send_channel in UdpBroadcaster"),
        )
    }
}