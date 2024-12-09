use std::time::SystemTime;

use crate::boat_state::BoatState;
use crate::utils::AISData;
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
    fn get_ais_data(&self) -> AISData {
        let lat_lon = self.get_current_position();
        AISData::new(self.course, lat_lon.0, lat_lon.1, self.mmsi.clone(), 1)
    }

    fn get_current_position(&self) -> (f32, f32) {
        if let Ok(timestamp) = SystemTime::now().duration_since(self.init_timestamp) {
            let seconds = timestamp.as_secs_f32();
            let lat = self.init_lat + f32::cos(self.course) / 3600.0 * seconds * self.speed;
            let lon = self.init_lon + f32::sin(self.course) / 3600.0 * seconds * self.speed;
            //Above is slightly suboptimal since sin / 3600 could be calculated once but idgaf
            (lat, lon)
        } else {
            (-1.0, -1.0) // ToDo
        }
    }

    fn get_current_course(&self) -> f32 {
        self.course
    }

    fn get_mmsi(&self) -> String {
        self.mmsi.clone() //mmsi is not a long string so it's not expensive clone
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
}
