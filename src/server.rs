use receiver::Receiver;

pub mod receiver;
pub mod tcp_server;
pub mod tcp_udp_server;

pub trait Server<T, Strong, Weak>
where
    T: Send + 'static,
    Strong: Receiver<T> + Send + 'static,
    Weak: Receiver<T> + Send + 'static
{
    fn get_strong_receiver(&mut self) -> Strong;
    fn get_weak_receiver(&mut self) -> Weak;
    // async fn strong_receiver_spawner(&mut self) {
    //     tokio::spawn(future)
    // }

    async fn run(&mut self) {
        let mut strong_receiver = self.get_strong_receiver();
        let mut weak_receiver = self.get_weak_receiver();
        let strong_handle = tokio::spawn(async move {
            strong_receiver.run().await;
        });

        let weak_handle = tokio::spawn(async move {
            weak_receiver.run().await;
        });
        let _ = tokio::join!(strong_handle, weak_handle);
    }
}
