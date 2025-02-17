use std::time::SystemTime;

use crate::boat_state::BoatState;
use crate::utils::{encode_ais_data, string_to_msg_type, AISData, MsgType};
/**
 * It's just a boat of fixed course and speed
 */
pub struct BoatStateMockup {
    init_lat: f32,
    init_lon: f32,
    course: f32,
    speed: f32, //in knots
    mmsi: String,
    init_timestamp: SystemTime,
}

impl BoatState for BoatStateMockup {
    async fn get_ais_data(&self) -> MsgType {
        let lat_lon = self.get_current_position();
        let data = AISData::new(self.course, lat_lon.0, lat_lon.1, self.mmsi.clone(), 1);
        string_to_msg_type(encode_ais_data(data).await.unwrap())
    }
}

impl BoatStateMockup {
    pub fn new(lat: f32, lon: f32, speed: f32, course: f32, mmsi: String) -> Self {
        BoatStateMockup {
            init_lat: lat,
            init_lon: lon,
            course,
            speed,
            mmsi,
            init_timestamp: SystemTime::now(),
        }
    }

    fn get_current_position(&self) -> (f32, f32) {
        if let Ok(timestamp) = SystemTime::now().duration_since(self.init_timestamp) {
            let seconds = timestamp.as_secs_f32();
            let lat = self.init_lat + f32::cos(self.course) / 3600.0 * seconds * self.speed;
            let lon = self.init_lon + f32::sin(self.course) / 3600.0 * seconds * self.speed;
            //Above is slightly suboptimal since sin / 3600 could be calculated once
            (lat, lon)
        } else {
            (-1.0, -1.0) // ToDo
        }
    }
}
