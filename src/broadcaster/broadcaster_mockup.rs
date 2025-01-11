use std::fmt::Display;

use tokio::sync::broadcast;

use crate::broadcaster::Broadcaster;

pub struct BroadcasterMockup<M> {
    send_channel: broadcast::Sender<M>,
    recv_channel: broadcast::Receiver<M>,
}

impl<M> Broadcaster<M, broadcast::Sender<M>, broadcast::Receiver<M>, ()> for BroadcasterMockup<M>
where
    M: Send + Copy + 'static,
{
    fn broadcast(
        arg: &mut broadcast::Sender<M>,
        msg: M,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let _ = arg.send(msg);
        }
    }

    fn recv_from_broadcast(
        arg: &mut broadcast::Receiver<M>,
    ) -> impl std::future::Future<Output = M> + Send {
        async move { arg.recv().await.unwrap() }
    }

    fn check_if_msg_already_passed(msg: M) -> bool {
        rand::random()
    }

    fn log_received_from_broadcast(arg: &mut (), msg: M) {
        println!("GOT MSG FROM BROADCAST");
    }
}

impl<M> BroadcasterMockup<M> {
    pub fn new(send_channel: broadcast::Sender<M>, recv_channel: broadcast::Receiver<M>) -> Self {
        Self {
            send_channel,
            recv_channel,
        }
    }
}
