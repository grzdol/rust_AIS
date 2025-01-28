use super::BroadcasterParams;
use crate::broadcaster::Broadcaster;
use crate::client::sender::udp_sender::UdpSender;
use crate::client::sender::Sender;
use crate::utils::{MsgType, MSGTYPESIZE};
use log::error;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

pub struct UdpBroadcasterParams {}

impl BroadcasterParams for UdpBroadcasterParams {
    type SenderArgs = (UdpSocket, SocketAddrV4);

    type ReceiverArgs = UdpSocket;

    type LoggerArgs = UdpSender;

    type B = UdpBroadcaster;
}

pub struct UdpBroadcaster {
    sender_socket: Option<UdpSocket>,
    receiver_socket: Option<UdpSocket>,
    log_arg: Option<UdpSender>,
    local_recv_channel: Option<mpsc::UnboundedReceiver<MsgType>>,
    local_send_channel: Option<mpsc::UnboundedSender<MsgType>>,
    multicast_address: SocketAddrV4,
}

impl UdpBroadcaster {
    pub async fn new(
        local_receiver_ip: Ipv4Addr,
        local_receiver_port: u16,
        _local_sender_ip: Ipv4Addr,
        _local_sender_port: u16,
        multicast_address: Ipv4Addr,
        multicast_port: u16,
        log_arg: UdpSender,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        println!(
            "Binding receiver socket to {}:{}",
            local_receiver_ip, local_receiver_port
        );
        let receiver_socket = match UdpSocket::bind(SocketAddrV4::new(
            local_receiver_ip,
            local_receiver_port,
        ))
        .await
        {
            Ok(socket) => socket,
            Err(e) => {
                error!("Failed to bind receiver socket: {}", e);
                return Err(Box::new(e));
            }
        };

        println!("Creating sender socket");
        let socket = match Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)) {
            Ok(socket) => socket,
            Err(e) => {
                error!("Failed to create sender socket: {}", e);
                return Err(Box::new(e));
            }
        };

        println!("Binding sender socket to any address");
        if let Err(e) = socket.bind(&SocketAddr::from(([0, 0, 0, 0], 0)).into()) {
            error!("Failed to bind sender socket: {}", e);
            return Err(Box::new(e));
        }

        println!("Setting multicast interface for sender socket");
        if let Err(e) = socket.set_multicast_if_v4(&multicast_address) {
            error!("Failed to set multicast interface: {}", e);
            return Err(Box::new(e));
        }

        let sender_socket = match UdpSocket::from_std(socket.into()) {
            Ok(socket) => socket,
            Err(e) => {
                error!("Failed to create sender socket from std socket: {}", e);
                return Err(Box::new(e));
            }
        };

        println!(
            "Joining multicast group {} on interface {}",
            multicast_address, local_receiver_ip
        );
        if let Err(e) = receiver_socket.join_multicast_v4(multicast_address, local_receiver_ip) {
            error!("Error joining multicast group: {}", e);
            return Err(Box::new(e));
        }

        let multicast_address = SocketAddrV4::new(multicast_address, multicast_port);

        Ok(Self {
            sender_socket: Some(sender_socket),
            receiver_socket: Some(receiver_socket),
            log_arg: Some(log_arg),
            local_recv_channel: None,
            local_send_channel: None,
            multicast_address,
        })
    }
}

impl Broadcaster<(UdpSocket, SocketAddrV4), UdpSocket, UdpSender> for UdpBroadcaster {
    async fn broadcast(arg: &mut (UdpSocket, SocketAddrV4), msg: MsgType) {
        let _ = arg.0.send_to(&msg, arg.1).await;
        // println!("sent to broadcast");
    }

    async fn recv_from_broadcast(socket: &mut UdpSocket) -> MsgType {
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

    async fn log_received_from_broadcast(sender: &mut UdpSender, msg: MsgType) {
        println!("logged msg");
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
        (UdpSocket, SocketAddrV4),
        UdpSender,
        mpsc::UnboundedReceiver<MsgType>,
        mpsc::UnboundedSender<MsgType>,
    ) {
        (
            self.receiver_socket
                .take()
                .expect("No receiver_socket in UdpBroadcaster"),
            (
                self.sender_socket
                    .take()
                    .expect("No sender_socket in UdpBroadcaster"),
                self.multicast_address,
            ),
            self.log_arg.take().expect("No log_arg in UdpBroadcaster"),
            self.local_recv_channel
                .take()
                .expect("No local_recv_channel in UdpBroadcaster"),
            self.local_send_channel
                .take()
                .expect("No local_send_channel in UdpBroadcaster"),
        )
    }
}
