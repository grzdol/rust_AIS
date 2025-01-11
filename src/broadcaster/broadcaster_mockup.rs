use std::{collections::HashSet, fmt::{Debug, Display}, hash::Hash};

use tokio::sync::broadcast;

use crate::broadcaster::Broadcaster;

pub struct BroadcasterMockup {

}

impl<M> Broadcaster<M, broadcast::Sender<M>, broadcast::Receiver<M>, ()> for BroadcasterMockup
where
    M: Send + Copy + Debug + Eq + Hash+ 'static,
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

    fn log_received_from_broadcast(_: &mut (), msg: M) {
        println!("GOT MSG FROM BROADCAST {:?}", msg);
    }
}
