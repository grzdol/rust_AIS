use futures::channel::mpsc::UnboundedReceiver;
use tokio::sync::mpsc;

/**
 * This is a mechanism of passing information between Clients.
 * We assume that a boat might not have internet connection, but
 * it may be able to broadcast data through radio to other boats, thats why we need it.
 */

pub trait Broadcaster<M:Send + Copy + 'static> {
    fn broadcast<T: Send>(arg: T, msg: M) -> impl std::future::Future<Output = ()> + Send;
    fn recv_from_broadcast<T: Send>(arg: T)
        -> impl std::future::Future<Output = M> + Send;
    /**
     * This method helps avoiding cycles in broadcast. If broadcaster receives msg second time,
     * it checks if it has already broadcasted this msg and doesnt propagate. It can be achived by hashset
     * on mmsi + timestamp.
     */
    fn check_if_msg_already_passed<T>(msg: T) -> bool;
    /**
     * We probably would like to somehow log data receved from broadcaster to client, even if we cant forward it
     * to other clients or server.
     */
    fn log_received_from_broadcast<T>(arg: T, msg: M);

    //worker for broadcasting data
    fn run_sender<T: Sync + Send>(
        arg: T,
        mut recv_channel: mpsc::UnboundedReceiver<M>,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            loop {
                let msg = recv_channel.recv().await.unwrap();
                //msg could be received from client or from receiver
                //hence we neeed to check if msg was already broadcasted
                if Self::check_if_msg_already_passed(msg) {
                    continue;
                }
                //here we simply copy msg since our msgs are pretty short, we accept that.
                Self::broadcast(&arg, msg).await;
            }
        }
    }
    //worker for receiving broadcast msgs from other clients
    fn run_receiver<T: Sync + Send>(
        arg: T,
        send_channel: mpsc::UnboundedSender<M>,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            loop {
                let msg: M = Self::recv_from_broadcast(&arg).await;
                Self::log_received_from_broadcast(&arg, msg);
                let _ = send_channel.send(msg);
            }
        }
    }
    /**
     * Below is deprecated end kinda dumb. Client can simply send pass msg to broadcasters sender through channel
     */
    //worker for receiving data from client and passing it to sender
    // fn recv_data_from_client<T: Send + 'static, V: Send>(
    //     arg: T, mut recv_channel: mpsc::UnboundedReceiver<V>, send_channel: mpsc::UnboundedSender<V>
    // ) -> impl std::future::Future<Output = ()> + Send {
    //   async move {
    //     loop {

    //     }
    //   }
    // }

    fn run<T, U, V>(
        receiver_args: T,
        sender_args: U,
        recv_channel: mpsc::UnboundedReceiver<M>,
        send_channel: mpsc::UnboundedSender<M>,
    ) -> impl std::future::Future<Output = ()> + Send
    where
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
    {
        async move {
          let receiver = tokio::spawn({
            async move {
              Self::run_receiver(receiver_args, send_channel).await;
            }
          });

          let sender = tokio::spawn({
            async move {
              Self::run_sender(sender_args, recv_channel).await;
            }
          });
          let _ = tokio::join!(receiver, sender);
        }
    }
}
