use std::collections::HashSet;

use futures::channel::mpsc::UnboundedReceiver;
use tokio::sync::mpsc;

use crate::utils::MsgType;
pub mod broadcaster_mockup;
use std::hash::Hash;

/**
 * This is a mechanism of passing information between Clients.
 * We assume that a boat might not have internet connection, but
 * it may be able to broadcast data through radio to other boats, thats why we need it.
 */

pub trait Broadcaster<MessageType, SenderArgs, ReceiverArgs, LoggerArgs>
where
    SenderArgs: Send + 'static,
    ReceiverArgs: Send + 'static,
    LoggerArgs: Send + 'static,
    MessageType: Send + Copy + Eq + Hash + 'static,
{
    fn broadcast(
        arg: &mut SenderArgs,
        msg: MessageType,
    ) -> impl std::future::Future<Output = ()> + Send;
    fn recv_from_broadcast(
        arg: &mut ReceiverArgs,
    ) -> impl std::future::Future<Output = MessageType> + Send;
    
    /**
     * We probably would like to somehow log data receved from broadcaster to client, even if we cant forward it
     * to other clients or server.
     */
    fn log_received_from_broadcast(arg: &mut LoggerArgs, msg: MessageType);

    /**
     * This method helps avoiding cycles in broadcast. If broadcaster receives msg second time,
     * it checks if it has already broadcasted this msg and doesnt propagate.
     */
    fn check_if_msg_already_passed(msg: MessageType, old_msg_set: &mut HashSet<MessageType>) -> bool {
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
        mut recv_channel: mpsc::UnboundedReceiver<MessageType>,
        mut old_msg_set: HashSet<MessageType>
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

    fn run(
        receiver_args: ReceiverArgs,
        sender_args: SenderArgs,
        log_arg: LoggerArgs,
        recv_channel: mpsc::UnboundedReceiver<MessageType>,
        send_channel: mpsc::UnboundedSender<MessageType>,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let receiver = tokio::spawn({
                async move {
                    Self::run_receiver(receiver_args, log_arg, send_channel).await;
                }
            });

            let sender = tokio::spawn({
                async move {
                    Self::run_sender(sender_args, recv_channel, HashSet::new()).await;
                }
            });
            let _ = tokio::join!(receiver, sender);
        }
    }
}
