/**
 *
 *
 * This module is for representing a boat state.
 * Since we would probably need more than one representation of this state,
 * ex. artificial state and like something that takes real data, which
 * we will probably do in more than one way, it's seems reasonable to define interface
 * for those states.
 *
 *
 */
use crate::utils::AISData;
pub mod boat_state_mockup;
pub trait BoatState: Send {
    fn get_ais_data(&self) -> AISData;
    fn get_current_position(&self) -> (f32, f32);
    fn get_current_course(&self) -> f32;
    fn get_mmsi(&self) -> String;
}
