use std::net::IpAddr;

use bytes::Bytes;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::task;

pub struct TcpServer {
    listener: TcpListener,
    opencpn_stream: TcpStream,
    tx: UnboundedSender<Bytes>,
    rx: UnboundedReceiver<Bytes>,
}

impl TcpServer {
    pub async fn new(
        ip: IpAddr,
        port: u16,
        opencpn_server_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = match TcpListener::bind((ip, port)).await {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind to {}:{}", ip, port);
                return Err(Box::new(e));
            }
        };

        let opencpn_stream = match TcpStream::connect(opencpn_server_address).await {
            Ok(stream) => {
                println!("Connected to OpenCPN at {}", opencpn_server_address);
                stream
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        };
        let (tx, rx) = mpsc::unbounded_channel::<Bytes>();

        Ok(TcpServer {
            listener,
            opencpn_stream,
            tx,
            rx,
        })
    }

    async fn handle_client(
        tx: UnboundedSender<Bytes>,
        mut socket: TcpStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = vec![0; 1024];
        loop {
            let n: usize = socket.read(&mut buf).await?;
            if n == 0 {
                break;
            }
            // ToDo, we shouldn't copy here
            let message = Bytes::from(buf[..n].to_vec());
            tx.send(message)?;
        }
        Ok(())
    }

    async fn accept_client_loop(
        tx: UnboundedSender<Bytes>,
        listener: TcpListener,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let (socket, addr) = listener.accept().await?;
            let tx2 = tx.clone();
            println!("New connection from: {}", addr);
            task::spawn(async move {
                if let Err(e) = TcpServer::handle_client(tx2, socket).await {
                    eprintln!("Error handling client {}: {}", addr, e);
                }
            });
        }
    }

    async fn opencpn_publisher(
        mut rx: UnboundedReceiver<Bytes>,
        mut opencpn_stream: TcpStream,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(message) = rx.recv().await {
            println!("Received message: {:?}", String::from_utf8_lossy(&message));

            // Send the NMEA sentence to the OpenCPN server
            // if let Err(e) = opencpn_stream.write_all(&message).await {
            //     eprintln!("Failed to write to stream: {}. Reconnecting...", e);
            //     break; // Exit loop to reconnect
            // }

            // // Flush the stream to ensure the message is sent immediately
            // if let Err(e) = opencpn_stream.flush().await {
            //     eprintln!("Failed to flush stream: {}. Reconnecting...", e);
            //     break; // Exit loop to reconnect
            // }

            // println!("Sent: {:?}", String::from_utf8_lossy(&message));
        }
        Ok(())
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let accept_loop = tokio::spawn(async move {
            let _ = TcpServer::accept_client_loop(self.tx, self.listener).await;
        });
        let publisher = tokio::spawn(async move {
            let _ = TcpServer::opencpn_publisher(self.rx, self.opencpn_stream).await;
        });
        tokio::join!(accept_loop, publisher);
        Ok(())
    }
}
