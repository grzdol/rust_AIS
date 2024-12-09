use crate::boat_state::{self, BoatState};
use crate::utils;
use std::net::IpAddr;
use std::thread::sleep;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
pub struct TcpClient<T>
where
    T: BoatState,
{
    ip: IpAddr,
    port: u16,
    stream: TcpStream,
    boat_state: T,
}

impl<T: BoatState> TcpClient<T> {
    pub async fn new(
        ip: IpAddr,
        port: u16,
        boat_state: T,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = match TcpStream::connect((ip, port)).await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to connect to {}:{}", ip, port);
                return Err(Box::new(e));
            }
        };
        Ok(TcpClient {
            ip,
            port,
            stream,
            boat_state,
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            //send AIS data every second
            let data = self.boat_state.get_ais_data();
            let encoded_ais_data = utils::encode_ais_data(data).await?;
            self.stream.write_all(encoded_ais_data.as_bytes()).await?;
            sleep(std::time::Duration::from_secs(1));
        }
    }
}
