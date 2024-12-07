use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // OpenCPN's TCP server details
    let server_address = "0.0.0.0:2137"; // Replace with your OpenCPN IP and port

    // Example NMEA sentence (GGA for GPS position)
    let nmea_sentence = b"!AIVDM,1,1,,A,14eG;o@034o8sd<L9i:a;WF>062D,0*7D";

    // Connect to OpenCPN server
    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Connected to OpenCPN at {}", server_address);

            loop {
                // Send NMEA sentence
                if let Err(e) = stream.write_all(nmea_sentence) {
                    eprintln!("Failed to send data: {}", e);
                    break;
                }
                // println!("Sent: {}", nmea_sentence);

                // Wait before sending the next sentence
                thread::sleep(Duration::from_secs(1));
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to OpenCPN: {}", e);
        }
    }
}
