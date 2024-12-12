use std::net::IpAddr;

use bytes::Bytes;
// use super::receiver::
use tokio::{
    io::AsyncWriteExt, net::{TcpListener, TcpStream, UdpSocket}, sync::mpsc::{self, UnboundedReceiver, UnboundedSender}
};
use crate::server::receiver::{tcp_receiver::TcpReceiver, udp_receiver::UdpReceiver};

use super::{receiver::{udp_receiver, Receiver}, tcp_server::TcpServer, Server};

pub struct TcpUdpServer {
    // tx: UnboundedSender<Bytes>,
    // rx: UnboundedReceiver<Bytes>,
    tcp_worker: TcpServer,
    udp_worker: UdpReceiver,
    rx: UnboundedReceiver<Bytes>
}

impl TcpUdpServer {
    pub async fn new(
        udp_ip: IpAddr,
        udp_port: u16,
        tcp_ip: IpAddr,
        tcp_port: u16,
        opencpn_server_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let udp_socket = match UdpSocket::bind((udp_ip, udp_port)).await {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind to {}:{}", udp_ip, udp_port);
                return Err(Box::new(e));
            }
        };

        let (tx, rx) = mpsc::unbounded_channel::<Bytes>();
        let tcp_worker = TcpServer::new(tcp_ip, tcp_port, opencpn_server_address).await.unwrap();
        let udp_worker = UdpReceiver::new(tx.clone(), udp_socket);
        Ok(TcpUdpServer {
            tcp_worker,
            udp_worker,
            rx
        })
    }

    pub async fn run(mut self) {
        let udp_handler = tokio::spawn(async move {
            let _ = self.udp_worker.run().await;
        });

        let tcp_handler = tokio::spawn(async move {
            let _ = self.tcp_worker.run().await;
        });

        //ToDo
        let opencpn_publisher = tokio::spawn(async move {
            let mut opencpn_stream = match TcpStream::connect("0.0.0.0:2137").await {
                Ok(stream) => {
                    // println!("Connected to OpenCPN at {}", opencpn_server_address);
                    stream
                }
                Err(e) => {
                    return Err(Box::new(e));
                }
            };
            Ok(while let Some(message) = self.rx.recv().await {
                println!("Received message: {:?}", String::from_utf8_lossy(&message));
    
                // Send the NMEA sentence to the OpenCPN server
                if let Err(e) = opencpn_stream.write_all(&message).await {
                    eprintln!("Failed to write to stream: {}. Reconnecting...", e);
                    break; // Exit loop to reconnect
                }
    
                // Flush the stream to ensure the message is sent immediately
                if let Err(e) = opencpn_stream.flush().await {
                    eprintln!("Failed to flush stream: {}. Reconnecting...", e);
                    break; // Exit loop to reconnect
                }
    
                println!("Sent: {:?}", String::from_utf8_lossy(&message));
            })
        });

        tokio::join!(udp_handler, tcp_handler);
    }
}
