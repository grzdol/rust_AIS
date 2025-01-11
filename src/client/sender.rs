use crate::utils::MsgType;

pub mod tcp_sender;
pub mod udp_sender;

pub trait Sender: Send + 'static {
    fn send(&mut self, msg: MsgType) -> impl std::future::Future<Output = ()> + Send;
}
