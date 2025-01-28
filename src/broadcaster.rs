use std::collections::HashSet;

use tokio::sync::mpsc;

use crate::utils::MsgType;
pub mod broadcaster_mockup;
pub mod udp_broadcaster;

/**
 * This is a mechanism of passing information between Clients.
 * We assume that a boat might not have internet connection, but
 * it may be able to broadcast data through radio to other boats, thats why we need it.
 * Since it's pretty unclear how exactly Broadcaster could be implemented, theres a lot of
 * generic Args
 */
pub trait BroadcasterParams: Send + 'static {
    type SenderArgs: Send + 'static; //Maybe its better to make them Copy
    type ReceiverArgs: Send + 'static;
    type LoggerArgs: Send + 'static;
    type B: Broadcaster<Self::SenderArgs, Self::ReceiverArgs, Self::LoggerArgs>;
}

pub trait Broadcaster<SenderArgs, ReceiverArgs, LoggerArgs>: Send + 'static
where
    SenderArgs: Send + 'static,
    ReceiverArgs: Send + 'static,
    LoggerArgs: Send + 'static,
{
    fn broadcast(
        arg: &mut SenderArgs,
        msg: MsgType,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn recv_from_broadcast(
        arg: &mut ReceiverArgs,
    ) -> impl std::future::Future<Output = MsgType> + Send;

    /**
     * We probably would like to somehow log data receved from broadcaster to client, even if we cant forward it
     * to other clients or server.
     */
    fn log_received_from_broadcast(
        arg: &mut LoggerArgs,
        msg: MsgType,
    ) -> impl std::future::Future<Output = ()> + std::marker::Send;

    fn set_recv_channel(
        &mut self,
        recv_channel: mpsc::UnboundedReceiver<MsgType>,
        send_channel: mpsc::UnboundedSender<MsgType>,
    );

    /**
     * This method helps avoiding cycles in broadcast. If broadcaster receives msg second time,
     * it checks if it has already broadcasted this msg and doesnt propagate.
     */
    fn check_if_msg_already_passed(msg: MsgType, old_msg_set: &mut HashSet<MsgType>) -> bool {
        if old_msg_set.contains(&msg) {
            true
        } else {
            old_msg_set.insert(msg);
            false
        }
    }
    //worker for broadcasting data
    fn run_sender(
        mut arg: SenderArgs,
        mut recv_channel: mpsc::UnboundedReceiver<MsgType>,
        mut old_msg_set: HashSet<MsgType>,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            loop {
                let msg = recv_channel.recv().await.unwrap();
                //msg could be received from client or from receiver
                //hence we neeed to check if msg was already broadcasted
                if Self::check_if_msg_already_passed(msg, &mut old_msg_set) {
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
        send_channel: mpsc::UnboundedSender<MsgType>,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            loop {
                let msg: MsgType = Self::recv_from_broadcast(&mut arg).await;
                Self::log_received_from_broadcast(&mut log_arg, msg).await;
                let _ = send_channel.send(msg);
            }
        }
    }

    fn get_args(
        &mut self,
    ) -> (
        ReceiverArgs,
        SenderArgs,
        LoggerArgs,
        mpsc::UnboundedReceiver<MsgType>,
        mpsc::UnboundedSender<MsgType>,
    );

    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send {
        let (receiver_args, sender_args, log_arg, recv_channel, send_channel) = self.get_args();

        async move {
            let receiver = tokio::spawn(async move {
                Self::run_receiver(receiver_args, log_arg, send_channel).await;
            });

            let sender = tokio::spawn(async move {
                Self::run_sender(sender_args, recv_channel, std::collections::HashSet::new()).await;
            });

            let _ = tokio::join!(receiver, sender);
        }
    }
}
