use std::time::Duration;

use sender::Sender;
use tokio::{
    sync::{
        broadcast,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
    },
    time::sleep,
};

use crate::{
    boat_state::BoatState,
    broadcaster::{self, Broadcaster, BroadcasterParams},
    utils::{
        build_timestamped_ais_message, encode_ais_data, string_to_msg_type, AISResponse, MsgType,
    },
};

pub mod sender;
pub mod tcp_client;
pub mod tcp_udp_client;

pub trait Client<T, WeakSender, StrongSender, BP>
where
    T: BoatState,
    WeakSender: Sender,
    StrongSender: Sender,
    BP: BroadcasterParams,
{
    fn get_broadcaster(
        &mut self,
        broadcaster_recv_channel: UnboundedReceiver<MsgType>,
    ) -> BP::B;

    fn boat_state_handler(
        boat_state: T,
        boat_state_channel: broadcast::Sender<MsgType>,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            loop {
                //ToDo this should be async call on boat state
                sleep(Duration::new(1, 0));
                let data = boat_state.get_ais_data();
                let encoded_data = encode_ais_data(data).await.unwrap();
                let timestamp = chrono::offset::Utc::now().to_rfc3339();
                let response = AISResponse {
                    timestamp,
                    ais_message: encoded_data,
                };
                let msg = build_timestamped_ais_message(response);
                let _ = boat_state_channel.send(string_to_msg_type(msg));
            }
        }
    }

    fn run_weak_sender(
        mut sender: WeakSender,
        mut boat_state_receiver: broadcast::Receiver<MsgType>,
        broadcaster_send_channel: UnboundedSender<MsgType>,
    ) -> impl std::future::Future<Output = ()> + std::marker::Send {
        async move {
            loop {
                match boat_state_receiver.recv().await {
                    Ok(msg) => {
                        sender.send(msg).await;
                        //Weak sender is responsible for passing msgs to broadcaster
                        broadcaster_send_channel.send(msg);
                    }
                    Err(e) => {
                        eprintln!("Error receiving boat state: {}", e);
                        break;
                    }
                }
            }
        }
    }

    fn run_strong_sender(
        mut sender: StrongSender,
        mut boat_state_receiver: broadcast::Receiver<MsgType>,
    ) -> impl std::future::Future<Output = ()> + std::marker::Send {
        async move {
            loop {
                match boat_state_receiver.recv().await {
                    Ok(msg) => {
                        sender.send(msg).await;
                    }
                    Err(e) => {
                        eprintln!("Error receiving boat state: {}", e);
                        break;
                    }
                }
            }
        }
    }

    async fn run(&mut self, weak_sender: WeakSender, strong_sender: StrongSender) {
        let (sender_channel, receiver_channel): (
            broadcast::Sender<MsgType>,
            broadcast::Receiver<MsgType>,
        ) = broadcast::channel(4096);
        let recv1 = sender_channel.subscribe();
        let strong_handle = tokio::spawn(async move {
            Self::run_strong_sender(strong_sender, recv1).await;
        });

        let (broadcaster_send_channel, broadcaster_recv_channel) = mpsc::unbounded_channel();

        let weak_handle = tokio::spawn(async move {
            Self::run_weak_sender(weak_sender, receiver_channel, broadcaster_send_channel).await;
        });

        let broadcaster = self.get_broadcaster(broadcaster_recv_channel).await;
        let broadcaster_handle = tokio::spawn(async move{
            broadcaster.run().await;
        });

        let _ = tokio::join!(strong_handle, weak_handle, broadcaster_handle);
    }
}
