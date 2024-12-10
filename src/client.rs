use serde::Serialize;
use tokio::task;

pub mod tcp_client;
pub trait Sender<T>: Send + Sync + 'static
where
    T: Serialize + Send + 'static,
{
    fn send(&mut self, data: T) -> impl std::future::Future<Output = ()> + Send;
    fn get_data(&mut self) -> impl std::future::Future<Output = T> + Send;

    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send {async {
        loop {
            let data = self.get_data().await;
            self.send(data).await;
        }
    } }
}

pub trait Client<T, S>:
where
    T: Serialize + Send + 'static,
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
