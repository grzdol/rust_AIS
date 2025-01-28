use sender::Sender;
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

use crate::{
    boat_state::BoatState,
    broadcaster::{Broadcaster, BroadcasterParams},
    utils::{build_timestamped_ais_message, string_to_msg_type, AISResponse, MsgType},
};

pub mod broken_client;
pub mod sender;
pub mod tcp_client;
pub mod tcp_udp_client;

pub trait Client<T, WeakSender, StrongSender, BP>
where
    T: BoatState + 'static,
    WeakSender: Sender,
    StrongSender: Sender,
    BP: BroadcasterParams,
    Self: Send,
{
    fn get_broadcaster(
        &mut self,
        broadcaster_recv_channel: UnboundedReceiver<MsgType>,
        broadcaster_send_channel: UnboundedSender<MsgType>,
    ) -> BP::B;
    fn get_boat_state(&mut self) -> T;
    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send;

    fn boat_state_handler(
        boat_state: T,
        boat_state_channel: broadcast::Sender<MsgType>,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            loop {
                let data = boat_state.get_ais_data().await;
                let trimmed_msg = String::from_utf8_lossy(&data)
                    .trim_end_matches(|c: char| c == '\0' || c.is_whitespace())
                    .to_string();
                let timestamp = chrono::offset::Utc::now().to_rfc3339();
                let response = AISResponse {
                    timestamp,
                    ais_message: trimmed_msg, // Use trim_end to remove trailing newline
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
                        let _ = broadcaster_send_channel.send(msg);
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

    fn run_impl(
        &mut self,
        weak_sender: WeakSender,
        strong_sender: StrongSender,
    ) -> impl std::future::Future<Output = ()> + Send {
        async {
            let (boat_state_sender_channel, _boat_state_receiver_channel): (
                broadcast::Sender<MsgType>,
                broadcast::Receiver<MsgType>,
            ) = broadcast::channel(4096);
            let recv1 = boat_state_sender_channel.subscribe();
            let recv2 = boat_state_sender_channel.subscribe();
            let strong_handle = tokio::spawn(async move {
                Self::run_strong_sender(strong_sender, recv1).await;
            });

            let (broadcaster_send_channel, broadcaster_recv_channel) = mpsc::unbounded_channel();

            let cp = broadcaster_send_channel.clone();
            let weak_handle = tokio::spawn(async move {
                Self::run_weak_sender(weak_sender, recv2, cp).await;
            });

            let mut broadcaster =
                self.get_broadcaster(broadcaster_recv_channel, broadcaster_send_channel);
            let broadcaster_handle = tokio::spawn(async move {
                broadcaster.run().await;
            });

            let boat_state = self.get_boat_state();
            let boat_state_publisher = tokio::spawn(async move {
                Self::boat_state_handler(boat_state, boat_state_sender_channel).await;
            });

            let _ = tokio::join!(
                strong_handle,
                weak_handle,
                broadcaster_handle,
                boat_state_publisher
            );
        }
    }
}
