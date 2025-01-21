use log::debug;
use rust_AIS::broadcaster::broadcaster_mockup::{BroadcasterMockup, BroadcasterMockupParams};
use rust_AIS::server::Server;
use rust_AIS::utils::MsgType;
use std::net::{IpAddr, Ipv4Addr};
use tokio::sync::broadcast;

use rust_AIS::boat_state::boat_state_mockup::BoatStateMockup;
use rust_AIS::broadcaster::BroadcasterParams;
use rust_AIS::client::sender::tcp_sender::TcpSender;
use rust_AIS::client::sender::udp_sender::UdpSender;
use rust_AIS::client::tcp_udp_client::TcpUdpClient;
use rust_AIS::client::Client;
use rust_AIS::server::tcp_udp_server::TcpUdpServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = TcpUdpServer::new(
        "127.0.0.1:6969",
        "127.0.0.1:4200",
        "0.0.0.0:2137",
        "0.0.0.0:2136",
    );

    let handle_server = tokio::spawn(async move {
        let _ = server.run().await;
    });

    let _ = tokio::join!(handle_server);
    Ok(())
}
