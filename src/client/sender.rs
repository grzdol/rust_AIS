use serde::Serialize;
pub mod tcp_strong_sender;
pub mod udp_weak_sender;

pub trait Sender<T>: Send + Sync + 'static
where
    T: Send + 'static,
{
    fn send(&mut self, data: T) -> impl std::future::Future<Output = ()> + Send;
    fn get_data(&mut self) -> impl std::future::Future<Output = T> + Send;

    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send {
        async {
            loop {
                let data = self.get_data().await;
                self.send(data).await;
            }
        }
    }
}
