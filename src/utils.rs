use reqwest::Client;
use serde::{Deserialize, Serialize};

type Error = Box<dyn std::error::Error>;
#[derive(Serialize)]
pub struct AISData {
    course: f32,
    lat: f32,
    long: f32,
    mmsi: String,
    message_type: i32,
}

impl AISData {
    pub fn new(course: f32, lat: f32, long: f32, mmsi: String, message_type: i32) -> Self {
        Self {
            course,
            lat,
            long,
            mmsi,
            message_type,
        }
    }
}

#[derive(Deserialize)]
struct AISResponse {
    ais_message: String,
}

pub async fn encode_ais_data(data: AISData) -> Result<String, Error> {
    //ToDo export to some config and use reqwest client
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:8000/generate_ais/")
        .json(&data)
        .send()
        .await?
        .json::<AISResponse>()
        .await?;
    Ok(response.ais_message)
}
