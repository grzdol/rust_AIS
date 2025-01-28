use std::net::Ipv4Addr;

use rust_AIS::boat_state::boat_state_udp::BoatStateUdp;
use rust_AIS::broadcaster::udp_broadcaster::{UdpBroadcaster, UdpBroadcasterParams};
use rust_AIS::client::sender::tcp_sender::TcpSender;
use rust_AIS::client::sender::udp_sender::UdpSender;
use rust_AIS::client::tcp_udp_client::TcpUdpClient;
use rust_AIS::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let boat_state_addr = "127.0.0.1:4321";
    let boat_state = BoatStateUdp::new(boat_state_addr).await;

    let multicast_addr = Ipv4Addr::new(192, 168, 0, 228);
    let client_multicast_addr = Ipv4Addr::new(224, 0, 0, 2);
    let multicast_port = 6789;
    let local_sender_ip = Ipv4Addr::new(127, 0, 0, 3);
    let local_sender_port = 5001;
    let sender = UdpSender::new("0.0.0.0:23456", "127.0.0.1:4200").await;
    let broadcaster = UdpBroadcaster::new(
        client_multicast_addr,
        multicast_port,
        local_sender_ip,
        local_sender_port,
        multicast_addr,
        multicast_port,
        sender,
    )
    .await
    .unwrap();

    let mut client = TcpUdpClient::<UdpBroadcasterParams, BoatStateUdp>::new(
        broadcaster,
        UdpSender::new("127.0.0.1:4211", "127.0.0.1:4200").await,
        TcpSender::new("127.0.0.1:6969").await,
        boat_state,
    );
    let handle = tokio::spawn(async move {
        client.run().await;
    });
    let _ = tokio::join!(handle);
    Ok(())
}
