use log::debug;
use std::net::{IpAddr, Ipv4Addr};

use rust_AIS::boat_state::boat_state_mockup::BoatStateMockup;
use rust_AIS::client::TcpUdpClient;
use rust_AIS::server::{self, TcpUdpServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let bornholm = (54.593406f32, 15.034773f32);
    let falsterbo = (55.229648f32, 12.489338f32);
    let falsterbo_to_swinoujscie_course = 141.0f32;
    let bornholm_to_swinoujscie_course = 191.0f32;

    let alefant = BoatStateMockup::new(
        falsterbo.0,
        falsterbo.1,
        13.3,
        falsterbo_to_swinoujscie_course,
        String::from("2137"),
    );
    let waternimf = BoatStateMockup::new(
        bornholm.0,
        bornholm.1,
        4.5,
        bornholm_to_swinoujscie_course,
        String::from("6969"),
    );
    debug!("afeafaafa");
    print!("afafafafafa");

    // let server = TcpUdpServer::new(
    //     localhost_v4,
    //     6969,
    //     "0.0.0.0:2137",
    //     "0.0.0.0:2136",
    //     localhost_v4,
    //     4200,
    // )
    // .await?;
    let server = TcpUdpServer::new(
        "127.0.0.1:6969",
        "127.0.0.1:4200",
        "0.0.0.0:2137",
        "0.0.0.0:2136",
    )
    .await?;
    print!("dupa");

    let handle_server = tokio::spawn(async move {
        let _ = server.run().await;
    });
    debug!("afaefaefaes");

    // let handle_alefant = tokio::spawn(async move {
    //     let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    //     let mut alefant_crew = TcpUdpClient::new(
    //         localhost_v4,
    //         6969,
    //         localhost_v4,
    //         4210,
    //         localhost_v4,
    //         4200,
    //         alefant,
    //     )
    //     .run()
    //     .await;
    // });
    let handle_alefant = tokio::spawn(async move {
        let mut alefant_crew = TcpUdpClient::new(
            "127.0.0.1:6969",
            "127.0.0.1:4210",
            "127.0.0.1:4200",
            alefant,
        );
        alefant_crew.run().await;
    });
    debug!("dupa1");

    let handle_waternimf = tokio::spawn(async move {
        let mut waternimf_crew = TcpUdpClient::new(
            "127.0.0.1:6969",
            "127.0.0.1:4211",
            "127.0.0.1:4200",
            waternimf,
        );
        waternimf_crew.run().await;
    });
    debug!("dupa14");

    tokio::join!(handle_server, handle_alefant, handle_waternimf);
    // tokio::join!(handle_server);
    Ok(())
}
