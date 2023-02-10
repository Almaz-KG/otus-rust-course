use crate::ServerResponse;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct TcpClient {
    host: String,
    port: u16,

    connection: Option<TcpStream>,
}

impl TcpClient {
    pub fn new(host: String, port: u16) -> Self {
        TcpClient {
            host,
            port,
            connection: None,
        }
    }

    fn read_data(socket: &mut TcpStream) -> Result<String, String> {
        let mut reader = BufReader::new(socket);
        let mut buf = [0; 4];
        reader.read_exact(&mut buf).unwrap();
        let len = u32::from_be_bytes(buf);

        let mut buf = vec![0; len as _];
        reader.read_exact(&mut buf).unwrap();
        let result = String::from_utf8(buf).map_err(|e| e.to_string())?;

        Ok(result)
    }

    fn write_data(socket: &mut TcpStream, data: &[u8]) -> Result<(), String> {
        socket
            .set_write_timeout(Some(Duration::from_secs(3)))
            .map_err(|e| format!("Unable to set write timeout: {:?}", e))?;

        socket
            .write_all(data)
            .map_err(|e| format!("Error: {:?}", e))?;
        Ok(())
    }

    pub fn connect(&mut self) -> ServerResponse {
        let mut stream = TcpStream::connect((self.host.clone(), self.port))
            .expect("Unable to connect to the host");

        TcpClient::write_data(&mut stream, b"handshake").unwrap();

        let handshake = TcpClient::read_data(&mut stream)?;

        if handshake.trim() != "handshake" {
            return Ok(format!("Expected handshake message, but got {}", handshake));
        }

        self.connection = Some(stream);

        Ok("Connection established.\n \
                Type --help to get detailed info. \
                Type `exit` or `quit` to exit from the app"
            .to_string())
    }

    pub fn command(&mut self, command: String) -> ServerResponse {
        if self.connection.is_none() {
            self.connect().unwrap();
        }

        let stream = self.connection.as_mut().unwrap();

        TcpClient::write_data(stream, command.as_bytes()).unwrap();
        let response = TcpClient::read_data(stream)?;
        Ok(response)
    }
}

pub struct UdpClient {
    host: String,
    port: u16,
}

impl UdpClient {
    pub fn new(host: String, port: u16) -> Self {
        UdpClient { host, port }
    }
}
