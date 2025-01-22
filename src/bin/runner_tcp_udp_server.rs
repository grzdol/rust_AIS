use log::debug;
use rust_AIS::boat_state::boat_state_udp::BoatStateUdp;
use rust_AIS::broadcaster::broadcaster_mockup::{BroadcasterMockup, BroadcasterMockupParams};
use rust_AIS::client::broken_client::BrokenClient;
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

    let (send_broadcast, recv_broadcast) = broadcast::channel::<MsgType>(1024);

    let alefant_udp_sender = UdpSender::new("127.0.0.1:4210", "127.0.0.1:4200").await;
    let alefant_tcp_sender = TcpSender::new("127.0.0.1:6969").await;
    let alefant = BoatStateUdp::new("127.0.0.1:4321").await;
    let broadcaster_logger = UdpSender::new("0.0.0.0:23456", "127.0.0.1:4200").await;
    let handle_alefant = tokio::spawn({
        let cp_send = send_broadcast.clone();
        let cp_recv = send_broadcast.subscribe();
        async move {
            let mut alefant_crew = TcpUdpClient::<BroadcasterMockupParams, BoatStateUdp>::new(
                BroadcasterMockup::new(cp_recv, cp_send, Some(broadcaster_logger)),
                alefant_udp_sender,
                alefant_tcp_sender,
                alefant,
            );
            alefant_crew.run().await;
        }
    });

    let bornholm = (54.593406f32, 15.034773f32);
    let bornholm_to_swinoujscie_course = 191.0f32;

    let waternimf = BoatStateMockup::new(
        bornholm.0,
        bornholm.1,
        5.0f32,
        bornholm_to_swinoujscie_course,
        String::from("12345"),
    );

    let handle_waternimf = tokio::spawn({
        let recv_broadcast = send_broadcast.subscribe();
        let send_broadcast = send_broadcast.clone();
        async move {
            let mut waternimf_crew = BrokenClient::<BroadcasterMockupParams, BoatStateMockup>::new(
                Some(BroadcasterMockup::new(recv_broadcast, send_broadcast, None)),
                Some(waternimf),
            );
            waternimf_crew.run().await;
        }
    });

    // Additional clients
    let bornholm_to_gedser_course = 45.0f32;

    let gedser = BoatStateMockup::new(
        bornholm.0,
        bornholm.1,
        6.0f32,
        bornholm_to_gedser_course,
        String::from("67890"),
    );

    let handle_gedser = tokio::spawn({
        let recv_broadcast = send_broadcast.subscribe();
        let send_broadcast = send_broadcast.clone();
        async move {
            let mut gedser_crew = TcpUdpClient::<BroadcasterMockupParams, BoatStateMockup>::new(
                BroadcasterMockup::new(recv_broadcast, send_broadcast, None),
                UdpSender::new("0.0.0.0:12351", "127.0.0.1:4200").await,
                TcpSender::new("127.0.0.1:6969").await,
                gedser,
            );
            gedser_crew.run().await;
        }
    });

    let bornholm_to_karlskrona_course = 315.0f32;

    let karlskrona = BoatStateMockup::new(
        bornholm.0,
        bornholm.1,
        7.0f32,
        bornholm_to_karlskrona_course,
        String::from("54321"),
    );

    let handle_karlskrona = tokio::spawn({
        let recv_broadcast = send_broadcast.subscribe();
        let send_broadcast = send_broadcast.clone();
        async move {
            let mut karlskrona_crew = BrokenClient::<BroadcasterMockupParams, BoatStateMockup>::new(
                Some(BroadcasterMockup::new(recv_broadcast, send_broadcast, None)),
                Some(karlskrona),
            );
            karlskrona_crew.run().await;
        }
    });

    let bornholm_to_gdansk_course = 90.0f32;

    let gdansk = BoatStateMockup::new(
        bornholm.0,
        bornholm.1,
        8.0f32,
        bornholm_to_gdansk_course,
        String::from("98765"),
    );

    let handle_gdansk = tokio::spawn({
        let recv_broadcast = send_broadcast.subscribe();
        let send_broadcast = send_broadcast.clone();
        async move {
            let mut gdansk_crew = BrokenClient::<BroadcasterMockupParams, BoatStateMockup>::new(
                Some(BroadcasterMockup::new(recv_broadcast, send_broadcast, None)),
                Some(gdansk),
            );
            gdansk_crew.run().await;
        }
    });

    let bornholm_to_rostok_course = 270.0f32;

    let rostok = BoatStateMockup::new(
        bornholm.0,
        bornholm.1,
        9.0f32,
        bornholm_to_rostok_course,
        String::from("11223"),
    );

    let handle_rostok = tokio::spawn({
        let recv_broadcast = send_broadcast.subscribe();
        let send_broadcast = send_broadcast.clone();
        async move {
            let mut rostok_crew = BrokenClient::<BroadcasterMockupParams, BoatStateMockup>::new(
                Some(BroadcasterMockup::new(recv_broadcast, send_broadcast, None)),
                Some(rostok),
            );
            rostok_crew.run().await;
        }
    });

    let bornholm_to_klaipeda_course = 135.0f32;

    let klaipeda = BoatStateMockup::new(
        bornholm.0,
        bornholm.1,
        10.0f32,
        bornholm_to_klaipeda_course,
        String::from("33445"),
    );

    let handle_klaipeda = tokio::spawn({
        let recv_broadcast = send_broadcast.subscribe();
        let send_broadcast = send_broadcast.clone();
        async move {
            let mut klaipeda_crew = BrokenClient::<BroadcasterMockupParams, BoatStateMockup>::new(
                Some(BroadcasterMockup::new(recv_broadcast, send_broadcast, None)),
                Some(klaipeda),
            );
            klaipeda_crew.run().await;
        }
    });

    let _ = tokio::join!(
        handle_server,
        handle_alefant,
        handle_waternimf,
        handle_gedser,
        handle_karlskrona,
        handle_gdansk,
        handle_rostok,
        handle_klaipeda
    );
    Ok(())
}
