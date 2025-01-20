use crate::{boat_state::BoatState, broadcaster::{Broadcaster, BroadcasterParams}};

use super::{sender::broken_sender::BrokenSedner, Client};

pub struct BrokenClient<BP: BroadcasterParams, T: BoatState> {
  broadcaster: Option<BP::B>,
  boat_state: Option<T>,
}

impl<T: BoatState + 'static, BP: BroadcasterParams> Client<T, BrokenSedner, BrokenSedner, BP> for BrokenClient<BP, T> {
    fn get_broadcaster(
        &mut self,
        broadcaster_recv_channel: tokio::sync::mpsc::UnboundedReceiver<crate::utils::MsgType>,
        broadcaster_send_channel: tokio::sync::mpsc::UnboundedSender<crate::utils::MsgType>,
    ) -> <BP as BroadcasterParams>::B {
      self.broadcaster
      .as_mut()
      .expect("No broadcaster. Panic")
      .set_recv_channel(broadcaster_recv_channel, broadcaster_send_channel);
      self.broadcaster.take().expect("No broadcaster. Panic")
    }

    fn get_boat_state(&mut self) -> T {
      self.boat_state.take().expect("No boat state set")
    }

    fn run(&mut self) -> impl std::future::Future<Output = ()> + Send {
        self.run_impl(BrokenSedner::new(), BrokenSedner::new())
    }
}