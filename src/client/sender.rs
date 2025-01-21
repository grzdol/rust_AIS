use crate::utils::MsgType;

pub mod broken_sender;
pub mod tcp_raw_nmea_sender;
pub mod tcp_sender;
pub mod udp_raw_nmea_sender;
pub mod udp_sender;

pub trait Sender: Send + 'static {
    fn send(&mut self, msg: MsgType) -> impl std::future::Future<Output = ()> + Send;
}
