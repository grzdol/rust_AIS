use std::net::{IpAddr, TcpListener};

/**
 * What server should be able to do right now:
 * 1. It should be able to handle multiple clients
 * 2. work in bad network environment
 * 3. host the received data somewhere
 *
 * And how should client work? I suppose it should somehow check sometimes if
 * it can send a packet.
 * So maybe I should simply consider creating a trait Client and trait Server.
 * Client trait should have methods:
 * 1. Gather_data
 * 2. Send_data
 * 3. Wait_for connection
 *
 *
 * Server should have methods:
 * 1. Handle_new_client
 * 2. Handle_connection
 * 3. Host_data?
 *
 * I suppose it should somehow talk with django server to get user credentials
 *
 *
 *
 *
 */
struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    fn new(addr: IpAddr, port: u16) -> Result<TcpServer, std::io::Error> {
        let listener = TcpListener::bind((addr, port))?;
        Ok(TcpServer { listener })
    }
}
