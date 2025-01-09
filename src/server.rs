pub mod tcp_server;
use std::net::IpAddr;

use bytes::Bytes;
use futures::io::Lines;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::task;
// use::codec::{Framed, LinesCodec};
// use tokio_utils::codec::{LinesCodec, Framed};
use tokio_util::codec::{Framed, FramedRead, LinesCodec};

use crate::utils::get_next_framed_ais_message;

pub struct TcpUdpServer {
    listener: TcpListener, //accepting new connections
    publishing_stream: TcpStream,
    tx: UnboundedSender<Bytes>,
    rx: UnboundedReceiver<Bytes>,
    udp_ip: IpAddr,
    udp_port: u16,
}

impl TcpUdpServer {
    pub async fn new(
        ip: IpAddr,
        port: u16,
        publishing_server_address: &str,
        udp_ip: IpAddr,
        udp_port: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = match TcpListener::bind((ip, port)).await {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind to {}:{}", ip, port);
                return Err(Box::new(e));
            }
        };

        let publishing_stream = match TcpStream::connect(publishing_server_address).await {
            Ok(stream) => {
                println!("Connected to OpenCPN at {}", publishing_server_address);
                stream
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        };
        let (tx, rx) = mpsc::unbounded_channel::<Bytes>();

        Ok(TcpUdpServer {
            listener,
            publishing_stream,
            tx,
            rx,
            udp_ip,
            udp_port,
        })
    }

    async fn handle_strong_sender(
        tx: UnboundedSender<Bytes>,
        mut socket: TcpStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = vec![0; 1024];
        let mut framed = FramedRead::new(socket, LinesCodec::new());
        loop {
            match get_next_framed_ais_message(&mut framed).await {
                Ok((message, timestamp)) => {
                    // handle the message and timestamp
                }
                Err(e) => {
                    eprintln!("Error getting AIS message: {}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    async fn accept_and_handle_strong(
        tx: UnboundedSender<Bytes>,
        listener: TcpListener,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let (socket, addr) = listener.accept().await?;
            let tx2 = tx.clone();
            println!("New connection from: {}", addr);
            task::spawn(async move {
                if let Err(e) = TcpUdpServer::handle_strong_sender(tx2, socket).await {
                    eprintln!("Error handling client {}: {}", addr, e);
                }
            });
        }
    }

    async fn handle_weak_senders(
        udp_ip: IpAddr,
        udp_port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let udp_socket = match UdpSocket::bind((udp_ip, udp_port)).await {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind to {}:{}", udp_ip, udp_port);
                return Err(Box::new(e));
            }
        };
        let mut buf = [0; 1024];

        loop {
            //We send one message in one datagram.
            let (len, _) = udp_socket.recv_from(&mut buf).await.unwrap();
            //todo
        }
        Ok(())
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let strong_receiver = tokio::spawn(async move {
            let _ = TcpUdpServer::accept_and_handle_strong(self.tx, self.listener).await;
        });
        let weak_receiver = tokio::spawn(async move {
            let _ = TcpUdpServer::handle_weak_senders(self.udp_ip, self.udp_port).await;
        });
        // let publisher = tokio::spawn(async move {
        //     let _ = TcpServer::opencpn_publisher(self.rx, self.opencpn_stream).await;
        // });
        tokio::join!(strong_receiver, weak_receiver);
        Ok(())
    }
}
