use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};

pub static TIMESTAMP: &str = "TIMESTAMP";
pub static MSGTYPESIZE: usize = 1024;
type Error = Box<dyn std::error::Error>;
pub type MsgType = [u8; MSGTYPESIZE];

#[derive(Serialize)]
pub struct AISData {
    course: f32,
    lat: f32,
    #[serde(rename = "lon")] 
    long: f32,
    mmsi: String,
    #[serde(rename = "type")] 
    message_type: i32,
    #[serde(rename = "talker_id")] 
    #[serde(default = "default_talker_id")] 
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
pub struct AISResponse {
    pub ais_message: String,
    pub timestamp: String, //isoformat
}

pub async fn encode_ais_data(data: AISData) -> Result<String, Error> {
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

pub async fn get_next_framed_ais_message(
    framed: &mut FramedRead<TcpStream, LinesCodec>,
) -> Result<(String, DateTime<Utc>), Box<dyn std::error::Error>> {
    if let Some(line) = framed.next().await {
        split_message_on_timestamp(line?)
    } else {
        Err("No more lines in the stream".into())
    }
}

pub fn split_message_on_timestamp(
    msg: String,
) -> Result<(String, DateTime<Utc>), Box<dyn std::error::Error>> {
    if let Some(pos) = msg.find(TIMESTAMP) {
        let (before, after) = msg.split_at(pos);
        let _timestamp_str = &after[TIMESTAMP.len()..].trim();
        // ToDo. Have no clue why below crashes for weak receiver
        // let timestamp = DateTime::parse_from_rfc3339(timestamp_str)?.with_timezone(&Utc);
        Ok((before.to_string(), Utc::now()))
    } else {
        println!("ERROR IN SPLITTING MSG {}", msg);
        Err(msg.into())
    }
}

pub fn build_timestamped_ais_message(data: AISResponse) -> String {
    data.ais_message.trim().to_owned() + TIMESTAMP + &data.timestamp
}

pub fn string_to_msg_type(s: String) -> MsgType {
    let mut msg_type = [0u8; MSGTYPESIZE];
    let bytes = s.as_bytes();
    let len = bytes.len().min(MSGTYPESIZE);
    msg_type[..len].copy_from_slice(&bytes[..len]);
    msg_type
}

pub fn msg_type_to_string(msg: MsgType) -> String {
    let len = msg.iter().position(|&x| x == 0).unwrap_or(msg.len());
    String::from_utf8_lossy(&msg[..len]).to_string()
}
