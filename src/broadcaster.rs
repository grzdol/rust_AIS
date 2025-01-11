use futures::channel::mpsc::UnboundedReceiver;
use tokio::sync::mpsc;
pub mod broadcaster_mockup;

/**
 * This is a mechanism of passing information between Clients.
 * We assume that a boat might not have internet connection, but
 * it may be able to broadcast data through radio to other boats, thats why we need it.
 */

pub trait Broadcaster<MessageType, SenderArgs, ReceiverArgs, LoggerArgs>
where
    SenderArgs: Send + Sync + 'static,
    ReceiverArgs: Send + Sync + 'static,
    LoggerArgs: Send + Sync + 'static,
    MessageType: Send + Copy + 'static
{
    fn broadcast(arg: &mut SenderArgs, msg: MessageType) -> impl std::future::Future<Output = ()> + Send;
    fn recv_from_broadcast(arg: &mut ReceiverArgs) -> impl std::future::Future<Output = MessageType> + Send;
    /**
     * This method helps avoiding cycles in broadcast. If broadcaster receives msg second time,
     * it checks if it has already broadcasted this msg and doesnt propagate. It can be achived by hashset
     * on mmsi + timestamp.
     */
    fn check_if_msg_already_passed(msg: MessageType) -> bool;
    /**
     * We probably would like to somehow log data receved from broadcaster to client, even if we cant forward it
     * to other clients or server.
     */
    fn log_received_from_broadcast(arg: &mut LoggerArgs, msg: MessageType);

    //worker for broadcasting data
    fn run_sender(
        mut arg: SenderArgs,
        mut recv_channel: mpsc::UnboundedReceiver<MessageType>,
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
                Self::broadcast(&mut arg, msg).await;
            }
        }
    }
    //worker for receiving broadcast msgs from other clients
    fn run_receiver(
        mut arg: ReceiverArgs,
        mut log_arg: LoggerArgs,
        send_channel: mpsc::UnboundedSender<MessageType>,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            loop {
                let msg: MessageType = Self::recv_from_broadcast(&mut arg).await;
                Self::log_received_from_broadcast(&mut log_arg, msg);
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

    fn run(
        receiver_args: ReceiverArgs,
        sender_args: SenderArgs,
        log_arg: LoggerArgs,
        recv_channel: mpsc::UnboundedReceiver<MessageType>,
        send_channel: mpsc::UnboundedSender<MessageType>,
    ) -> impl std::future::Future<Output = ()> + Send
    {
        async move {
            let receiver = tokio::spawn({
                async move {
                    Self::run_receiver(receiver_args, log_arg, send_channel).await;
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
