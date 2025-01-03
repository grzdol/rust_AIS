use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

type Error = Box<dyn std::error::Error>;
#[derive(Serialize)]
pub struct AISData {
    course: f32,
    lat: f32,
    #[serde(rename = "lon")] // Rename `long` to `lon` for serialization
    long: f32,
    mmsi: String,
    #[serde(rename = "type")] // Rename `message_type` to `type` for serialization
    message_type: i32,
    #[serde(rename = "talker_id")] // Include static field
    #[serde(default = "default_talker_id")] // Use a default function for talker_id
    talker_id: String,
}

impl AISData {
    pub fn new(course: f32, lat: f32, long: f32, mmsi: String, message_type: i32) -> Self {
        Self {
            course,
            lat,
            long,
            mmsi,
            message_type,
            talker_id: "AIVDM".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct AISResponse {
    ais_message: String,
}

pub async fn encode_ais_data(data: AISData) -> Result<String, Error> {
    //ToDo export to some config and use reqwest client
    println!("{}", json!(&data));
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
