pub mod udp_receiver;
pub mod tcp_receiver;

pub trait Receiver<T>: Send + Sync + 'static
where
    T: Send + 'static,
{
    fn recv(&mut self) -> impl std::future::Future<Output = T> + Send;
    fn publish_data(&mut self, data: T) -> impl std::future::Future<Output = ()> + Send;

    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send {
        async {
            loop {
                let data = self.recv().await;
                self.publish_data(data);
            }
        }
    }
}
