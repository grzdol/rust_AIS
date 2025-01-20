use super::Sender;

pub struct BrokenSedner {

}

impl BrokenSedner {
    pub fn new() -> Self {
        Self {  }
    }
}

impl Sender for BrokenSedner {
    fn send(&mut self, _msg: crate::utils::MsgType) -> impl std::future::Future<Output = ()> + Send {
        async {
          //it's broken it does nothing
        }
    }
}