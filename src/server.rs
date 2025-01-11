pub mod tcp_server;
use std::net::IpAddr;

use bytes::Bytes;
use futures::io::Lines;
use futures::SinkExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::task;
use tokio_stream::StreamExt;
// use::codec::{Framed, LinesCodec};
// use tokio_utils::codec::{LinesCodec, Framed};
use tokio_util::codec::{Framed, FramedRead, FramedWrite, LinesCodec};

use crate::utils::{get_next_framed_ais_message, split_message_on_TIMESTAMP};

pub struct TcpUdpServer {
    strong_listener: TcpListener,           //accepting new connections
    weak_receiever: UdpSocket,              //receiveing data from weak senders
    real_time_publishing_stream: TcpStream, //here we publish data received from weak sender
    history_publishing_stream: TcpStream,   //here we publish data received from strong sender
}

impl TcpUdpServer {
    pub async fn new(
        listener_addr: &str,
        udp_receiver_addr: &str,
        real_time_server_address: &str,
        history_server_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(listener_addr).await?;
        let real_time_publishing_stream = TcpStream::connect(real_time_server_address).await?;
        let history_publishing_stream = TcpStream::connect(history_server_address).await?;
        let udp_sock = UdpSocket::bind(udp_receiver_addr).await?;

        Ok(TcpUdpServer {
            strong_listener: listener,
            weak_receiever: udp_sock,
            real_time_publishing_stream,
            history_publishing_stream,
        })
    }

    async fn handle_strong_sender(
        tx: UnboundedSender<Bytes>,
        socket: TcpStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut framed = FramedRead::new(socket, LinesCodec::new());
        loop {
            let msg = match framed.next().await {
                Some(Ok(line)) => line,
                Some(Err(e)) => {
                    eprintln!("Error receiving line: {}", e);
                    break;
                }
                None => {
                    eprintln!("Stream ended.");
                    break;
                }
            };
            println!("STRONG SENDER MSG {}", msg);
            let _ = tx.send(Bytes::from(msg));
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
        udp_socket: UdpSocket,
        tx: UnboundedSender<Bytes>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = [0; 1024];
        loop {
            let (len, _) = match udp_socket.recv_from(&mut buf).await {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error receiving from UDP socket: {}", e);
                    break;
                }
            };
            println!("AAAAAAAAAAAAAAAa {:?} {}", buf, len);
            if let Err(e) = tx.send(Bytes::copy_from_slice(&buf[..len])) {
                eprintln!("Error sending message: {}", e);
                break;
            }
        }
        Ok(())
    }

    async fn data_publisher(
        mut framed: FramedWrite<TcpStream, LinesCodec>,
        mut receiver: mpsc::UnboundedReceiver<Bytes>,
    ) {
        while let Some(data) = receiver.recv().await {
            let msg = match String::from_utf8(data.to_vec()) {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("Error converting data to string: {}", e);
                    continue;
                }
            };

            let (ais_message, timestamp) = match split_message_on_TIMESTAMP(msg.to_string()) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error splitting message on TIMESTAMP: {}", e);
                    continue;
                }
            };

            if let Err(e) = framed.send(ais_message + "\n").await {
                eprintln!("Error sending message: {}", e);
            }
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let (strong_send, strong_recv) = mpsc::unbounded_channel::<Bytes>();
        let (weak_send, weak_recv) = mpsc::unbounded_channel::<Bytes>();

        let strong_receiver = tokio::spawn(async move {
            let _ = TcpUdpServer::accept_and_handle_strong(strong_send, self.strong_listener).await;
        });

        let weak_receiver = tokio::spawn(async move {
            let _ = TcpUdpServer::handle_weak_senders(self.weak_receiever, weak_send).await;
        });

        let strong_publisher = tokio::spawn(async move {
            let _ = TcpUdpServer::data_publisher(
                FramedWrite::new(self.history_publishing_stream, LinesCodec::new()),
                strong_recv,
            )
            .await;
        });

        let weak_publisher = tokio::spawn(async move {
            let _ = TcpUdpServer::data_publisher(
                FramedWrite::new(self.real_time_publishing_stream, LinesCodec::new()),
                weak_recv,
            )
            .await;
        });

        let _ = tokio::join!(
            strong_receiver,
            weak_receiver,
            strong_publisher,
            weak_publisher
        );
        Ok(())
    }
}
