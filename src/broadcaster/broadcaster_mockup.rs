use tokio::sync::{broadcast, mpsc};

use crate::broadcaster::Broadcaster;
use crate::client::sender::udp_sender::UdpSender;
use crate::client::sender::Sender;
use crate::utils::MsgType;

use super::BroadcasterParams;

pub struct BroadcasterMockupParams;

impl BroadcasterParams for BroadcasterMockupParams {
    type SenderArgs = broadcast::Sender<MsgType>;
    type ReceiverArgs = broadcast::Receiver<MsgType>;
    type LoggerArgs = Option<UdpSender>;
    type B = BroadcasterMockup;
}

pub struct BroadcasterMockup {
    receiver_arg: Option<broadcast::Receiver<MsgType>>, // broadcast channel where clients send
    sender_arg: Option<broadcast::Sender<MsgType>>,
    log_arg: Option<UdpSender>,
    local_recv_channel: Option<mpsc::UnboundedReceiver<MsgType>>, // channel for receiving data from client
    /**
     * when we receive data from broadcast,
     * we pass it to local_recv_channel so that it can be broadcasted further
     */
    local_send_channel: Option<mpsc::UnboundedSender<MsgType>>,
}

impl BroadcasterMockup {
    pub fn new(
        receiver_arg: broadcast::Receiver<MsgType>,
        sender_arg: broadcast::Sender<MsgType>,
        log_arg: Option<UdpSender>,
    ) -> Self {
        Self {
            receiver_arg: Some(receiver_arg),
            sender_arg: Some(sender_arg),
            log_arg,
            // below channels are set by client
            local_recv_channel: None,
            local_send_channel: None,
        }
    }
}

impl Broadcaster<broadcast::Sender<MsgType>, broadcast::Receiver<MsgType>, Option<UdpSender>>
    for BroadcasterMockup
{
    async fn broadcast(arg: &mut broadcast::Sender<MsgType>, msg: MsgType) {
        let _ = arg.send(msg);
    }

    async fn recv_from_broadcast(arg: &mut broadcast::Receiver<MsgType>) -> MsgType {
        arg.recv().await.unwrap()
    }

    async fn log_received_from_broadcast(sender: &mut Option<UdpSender>, msg: MsgType) {
        if let Some(sender) = sender {
            sender.send(msg).await;
            // println!("GOOD SEDNER {:#?}", msg);
        } else {
            // println!("BAD SEDNER {:#?}", msg);
        }
    }

    fn set_recv_channel(
        &mut self,
        recv_channel: mpsc::UnboundedReceiver<MsgType>,
        send_channel: mpsc::UnboundedSender<MsgType>,
    ) {
        self.local_recv_channel = Some(recv_channel);
        self.local_send_channel = Some(send_channel);
    }

    fn get_args(
        &mut self,
    ) -> (
        broadcast::Receiver<MsgType>,
        broadcast::Sender<MsgType>,
        Option<UdpSender>,
        mpsc::UnboundedReceiver<MsgType>,
        mpsc::UnboundedSender<MsgType>,
    ) {
        (
            self.receiver_arg
                .take()
                .expect("No receiver_arg in BroadcasterMockup"),
            self.sender_arg
                .take()
                .expect("No sender_arg in BroadcasterMockup"),
            self.log_arg.take(),
            self.local_recv_channel
                .take()
                .expect("No local_recv_channel in BroadcasterMockup"),
            self.local_send_channel
                .take()
                .expect("No local_send_channel in BroadcasterMockup"),
        )
    }
}
