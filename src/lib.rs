pub mod boat_state;
pub mod broadcaster;
pub mod client;
pub mod server;
pub mod utils;
// use std::net::{IpAddr, Ipv4Addr};

// use boat_state::boat_state_mockup::BoatStateMockup;
// use client::tcp_client;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
//     let server = server::tcp_server::TcpServer::new(localhost_v4, 6969, "0.0.0.0:2137").await?;

//     let bornholm = (54.593406f32, 15.034773f32);
//     let falsterbo = (55.229648f32, 12.489338f32);
//     let falsterbo_to_swinoujscie_course = 141.0f32;
//     let bornholm_to_swinoujscie_course = 191.0f32;

//     let alefant = BoatStateMockup::new(
//         falsterbo.0,
//         falsterbo.1,
//         13.3,
//         falsterbo_to_swinoujscie_course,
//         String::from("2137"),
//     );
//     let waternimf = BoatStateMockup::new(
//         bornholm.0,
//         bornholm.1,
//         4.5,
//         bornholm_to_swinoujscie_course,
//         String::from("6969"),
//     );

//     let handle_server = tokio::spawn(async move {
//         let _ = server.run().await;
//     });

//     let handle_alefant = tokio::spawn(async move {
//         let mut alefant_crew = tcp_client::TcpClient::new(localhost_v4, 6969, alefant)
//             .await
//             .unwrap();
//         let _ = alefant_crew.run().await;
//     });

//     let handle_waternimf = tokio::spawn(async move {
//         let mut waternimf_crew = tcp_client::TcpClient::new(localhost_v4, 6969, waternimf)
//             .await
//             .unwrap();
//         let _ = waternimf_crew.run().await;
//     });

//     tokio::join!(handle_alefant, handle_server, handle_waternimf);

//     Ok(())
// }
