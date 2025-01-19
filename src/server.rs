pub mod receiver;
pub mod tcp_server;
pub mod tcp_udp_server;
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

use crate::client::sender::Sender;
use crate::utils::{get_next_framed_ais_message, split_message_on_TIMESTAMP, MsgType};

use receiver::{Receiver, ReceiverT};

pub trait Server<WeakReceiver, StrongReceiver, StrongPublisher, WeakPublisher>
where
    WeakReceiver: ReceiverT,
    StrongReceiver: ReceiverT,
    StrongPublisher: Sender,
    WeakPublisher: Sender,
{
    async fn get_strong_receiver(&mut self) -> StrongReceiver::Receiver;
    async fn get_weak_receiver(&mut self) -> WeakReceiver::Receiver;
    async fn get_strong_publisher(&mut self) -> StrongPublisher;
    async fn get_weak_publisher(&mut self) -> WeakPublisher;

    fn data_publisher<S: Sender>(
        mut sender: S,
        mut receiver: UnboundedReceiver<MsgType>,
    ) -> impl std::future::Future<Output = ()> + std::marker::Send {
        async move {
            while let Some(msg) = receiver.recv().await {
                sender.send(msg).await;
            }
            println!("SRAKAAA");
        }
    }

    async fn run(&mut self) {
        let (strong_send, strong_recv) = mpsc::unbounded_channel::<MsgType>();
        let (weak_send, weak_recv) = mpsc::unbounded_channel::<MsgType>();

        let mut strong_receiver = self.get_strong_receiver().await;
        let mut weak_receiver = self.get_weak_receiver().await;
        let strong_publisher = self.get_strong_publisher().await;
        let weak_publisher = self.get_weak_publisher().await;

        let strong_receiver_handle = tokio::spawn(async move {
            let _ = strong_receiver.run(strong_send).await;
        });

        let weak_receiver_handle = tokio::spawn(async move {
            let _ = weak_receiver.run(weak_send).await;
        });

        let stron_data_publisher = tokio::spawn(async move {
            Self::data_publisher(strong_publisher, strong_recv).await;
        });

        let weak_data_publisher = tokio::spawn(async move {
            Self::data_publisher(weak_publisher, weak_recv).await;
        });

        let _ = tokio::join!(
            strong_receiver_handle,
            weak_receiver_handle,
            stron_data_publisher,
            weak_data_publisher
        );
    }
}
