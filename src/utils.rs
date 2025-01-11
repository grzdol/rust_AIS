use std::string;

use chrono::{DateTime, Utc};
use futures::sink::SinkExt;
use futures::AsyncRead;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{net::TcpStream, time};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, FramedRead, FramedWrite, LinesCodec};

type Error = Box<dyn std::error::Error>;
pub type MsgType = [u8; 1024];

static TIMESTAMP: &str = "TIMESTAMP";
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
pub struct AISResponse {
    pub ais_message: String,
    pub timestamp: String, //isoformat
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

pub async fn get_next_framed_ais_message(
    framed: &mut FramedRead<TcpStream, LinesCodec>,
) -> Result<(String, DateTime<Utc>), Box<dyn std::error::Error>> {
    if let Some(line) = framed.next().await {
        return split_message_on_TIMESTAMP(line?);
    } else {
        Err("No more lines in the stream".into())
    }
}

pub fn split_message_on_TIMESTAMP(
    msg: String,
) -> Result<(String, DateTime<Utc>), Box<dyn std::error::Error>> {
    if let Some(pos) = msg.find(TIMESTAMP) {
        let (before, after) = msg.split_at(pos);
        let timestamp_str = &after[TIMESTAMP.len()..].trim();
        let timestamp = DateTime::parse_from_rfc3339(timestamp_str)?.with_timezone(&Utc);
        Ok((before.to_string(), timestamp))
    } else {
        // println!("{}", msg);
        Err(msg.into())
    }
}

pub fn build_timestamped_ais_message(data: AISResponse) -> String {
    data.ais_message + TIMESTAMP + &data.timestamp
}



pub fn string_to_msg_type(s: String) -> MsgType {
    let mut msg_type = [0u8; 1024];
    let bytes = s.as_bytes();
    let len = bytes.len().min(1024);
    msg_type[..len].copy_from_slice(&bytes[..len]);
    msg_type
}