mod boat_state;
mod client;
mod server;
mod single_thread_tcp_server;
mod utils;
use std::net::{IpAddr, Ipv4Addr};

use boat_state::boat_state_mockup::{self, BoatStateMockup};
use client::tcp_client;
use futures::future::join_all;
use server::tcp_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let server = server::tcp_server::TcpServer::new(localhost_v4, 6969, "0.0.0.0:2137").await?;
    

    let bornholm = (5459.3406f32, 01503.4773f32);
    let falsterbo = (5522.9648f32, 01248.9338f32);
    let falsterbo_to_swinoujscie_course = 141.0f32;
    let bornholm_to_swinoujscie_course = 191.0f32;

    let alefant = BoatStateMockup::new(
        falsterbo.0,
        falsterbo.1,
        13.3,
        falsterbo_to_swinoujscie_course,
        String::from("ALEFANT"),
    );
    let waternimf = BoatStateMockup::new(
        bornholm.0,
        bornholm.1,
        4.5,
        bornholm_to_swinoujscie_course,
        String::from("WATERNIMF"),
    );

    let mut handles = Vec::new();

    let handle_server = tokio::spawn(async move {
        let _ = server.run();
    });

    let handle_alefant = tokio::spawn(async move {
        let mut alefant_crew = tcp_client::TcpClient::new(localhost_v4, 6969, alefant).await.unwrap();
        let _ = alefant_crew.run();
    });

    let handle_waternimf = tokio::spawn(async move {
        let mut waternimf_crew = tcp_client::TcpClient::new(localhost_v4, 6969, waternimf).await.unwrap();
        let _ = waternimf_crew.run();
    });

    
    handles.push(handle_server);
    handles.push(handle_waternimf);
    handles.push(handle_alefant);

    join_all(handles).await;


    Ok(())
}
