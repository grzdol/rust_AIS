use serde::Serialize;
use tokio::task;
mod sender;
use sender::Sender;
pub mod tcp_client;
pub mod tcp_udp_client;


pub trait Client<T, S>:
where
    T: Send + 'static,
    S: Sender<T> + Send + Sync + 'static,
{
    fn get_weak_sender(&mut self) -> S;
    fn get_strong_sender(&mut self) -> S;

    async fn run(&mut self) {
        let mut strong_sender = self.get_strong_sender();
        let mut weak_sender = self.get_weak_sender();
        let strong_handle = tokio::spawn(async move {
            strong_sender.run().await;
        });

        let weak_handle = tokio::spawn(async move {
            weak_sender.run().await;
        });
        let _ = tokio::join!(strong_handle, weak_handle);
    }
}
