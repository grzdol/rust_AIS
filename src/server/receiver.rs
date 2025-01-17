use tokio::{sync::mpsc, task::JoinHandle};

use crate::utils::MsgType;
pub mod tcp_receiver;
pub mod udp_receiver;

pub trait ReceiverT: Send + 'static {
    type AcceptArgs: Send + Sync + 'static;
    type Receiver: Receiver<Self::AcceptArgs>;
}

pub trait Receiver<AcceptArgs: Send + Sync + 'static>: Send + 'static {
    fn accept_client(&mut self) -> impl std::future::Future<Output = (AcceptArgs)> + Send;
    fn recv(args: AcceptArgs) -> impl std::future::Future<Output = (MsgType)> + Send;
    fn finish_accepting(&self) -> bool;

    fn run(
        &mut self,
        channel: mpsc::UnboundedSender<MsgType>,
    ) -> impl std::future::Future<Output = ()> + std::marker::Send {
        async move {
            let mut handles = Vec::<JoinHandle<()>>::new();
            while !self.finish_accepting() {
                let mut args = self.accept_client().await;
                let c = channel.clone();
                let handle = tokio::spawn(async move {
                    let msg = Self::recv(args).await;
                    c.send(msg);
                });
                handles.push(handle);
            }
            futures::future::join_all(handles).await;
        }
    }
}
