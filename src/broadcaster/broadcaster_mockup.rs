use std::fmt::Display;

use tokio::sync::broadcast;

use crate::broadcaster::Broadcaster;

pub struct BroadcasterMockup {}

impl<M> Broadcaster<M, broadcast::Sender<M>, broadcast::Receiver<M>, ()> for BroadcasterMockup
where
    M: Send + Copy + Display + 'static,
{
    fn broadcast(arg: &mut broadcast::Sender<M>, msg: M) -> impl std::future::Future<Output = ()> + Send {
        async move {
          let _ = arg.send(msg);
        }
    }

    fn recv_from_broadcast(arg: &mut broadcast::Receiver<M>) -> impl std::future::Future<Output = M> + Send {
        async move {
          arg.recv().await.unwrap()
        }
    }

    fn check_if_msg_already_passed(msg: M) -> bool {
        rand::random()
    }

    fn log_received_from_broadcast(arg: &mut (), msg: M) {
        println!("{}", msg);
    }
}
