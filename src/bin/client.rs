use rust_AIS::boat_state::boat_state_udp::BoatStateUdp;
use rust_AIS::broadcaster::udp_broadcaster::{UdpBroadcaster, UdpBroadcasterParams};
use rust_AIS::client::sender::tcp_raw_nmea_sender::TcpRawNmeaSender;
use rust_AIS::client::sender::tcp_sender::TcpSender;
use rust_AIS::client::sender::udp_sender::UdpSender;
use rust_AIS::client::tcp_udp_client::TcpUdpClient;
use rust_AIS::client::Client;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let boat_state_addr = "127.0.0.1:4321";
    // let broadcaster = UdpBroadcaster::new();
    let boat_state = BoatStateUdp::new(boat_state_addr).await;
    // let client = TcpUdpClient::new()
    let broadcaster = UdpBroadcaster::new(
        "0.0.0.0:5000",
        "0.0.0.0:5001",
        "192.168.2.255:3000",
        TcpRawNmeaSender::new("127.0.0.1:6969").await,
    ).await;
    let mut client = TcpUdpClient::<UdpBroadcasterParams, BoatStateUdp>::new(broadcaster,
        UdpSender::new("127.0.0.1:4211", "127.0.0.1:4200").await,
        TcpSender::new("127.0.0.1:6969").await,
        boat_state
    );
    let handle = tokio::spawn(async move {
        client.run().await;
    });
    tokio::join!(handle);
    Ok(())
}
