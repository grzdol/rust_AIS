use std::{
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
};
use tokio::sync::{broadcast, mpsc};

use crate::broadcaster::Broadcaster;

pub struct BroadcasterMockup<M> {
    receiver_arg: Option<broadcast::Receiver<M>>,
    sender_arg: Option<broadcast::Sender<M>>,
    log_arg: Option<()>,
    local_recv_channel: Option<mpsc::UnboundedReceiver<M>>,
    local_send_channel: Option<mpsc::UnboundedSender<M>>,
}

impl<M> BroadcasterMockup<M> {
    pub fn new(
        receiver_arg: broadcast::Receiver<M>,
        sender_arg: broadcast::Sender<M>,
        log_arg: (),
        local_recv_channel: mpsc::UnboundedReceiver<M>,
        local_send_channel: mpsc::UnboundedSender<M>,
    ) -> Self {
        Self {
            receiver_arg: Some(receiver_arg),
            sender_arg: Some(sender_arg),
            log_arg: Some(log_arg),
            local_recv_channel: Some(local_recv_channel),
            local_send_channel: Some(local_send_channel),
        }
    }
}

impl<M> Broadcaster<M, broadcast::Sender<M>, broadcast::Receiver<M>, ()> for BroadcasterMockup<M>
where
    M: Send + Copy + Debug + Eq + Hash + 'static,
{
    fn broadcast(arg: &mut broadcast::Sender<M>, msg: M) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let _ = arg.send(msg);
        }
    }

    fn recv_from_broadcast(arg: &mut broadcast::Receiver<M>) -> impl std::future::Future<Output = M> + Send {
        async move { arg.recv().await.unwrap() }
    }

    fn log_received_from_broadcast(_: &mut (), msg: M) {
        println!("GOT MSG FROM BROADCAST {:?}", msg);
    }

    fn set_recv_channel(&mut self, recv_channel: mpsc::UnboundedReceiver<M>) {
        self.local_recv_channel = Some(recv_channel);
    }

    fn get_args(
        &mut self,
    ) -> (
        broadcast::Receiver<M>,
        broadcast::Sender<M>,
        (),
        mpsc::UnboundedReceiver<M>,
        mpsc::UnboundedSender<M>,
    ) {
        (
            self.receiver_arg.take().expect("No receiver_arg in BroadcasterMockup"),
            self.sender_arg.take().expect("No sender_arg in BroadcasterMockup"),
            self.log_arg.take().expect("No log_arg in BroadcasterMockup"),
            self.local_recv_channel.take().expect("No local_recv_channel in BroadcasterMockup"),
            self.local_send_channel.take().expect("No local_send_channel in BroadcasterMockup"),
        )
    }
}
