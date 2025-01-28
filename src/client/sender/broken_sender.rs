use super::Sender;

pub struct BrokenSedner {}

impl Default for BrokenSedner {
    fn default() -> Self {
        Self::new()
    }
}

impl BrokenSedner {
    pub fn new() -> Self {
        Self {}
    }
}

impl Sender for BrokenSedner {
    async fn send(&mut self, _msg: crate::utils::MsgType) {
        //it's broken it does nothing
    }
}
