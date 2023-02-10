use crate::ServerResponse;
use std::io::{BufReader, Read, Write};
use std::net::{TcpStream, UdpSocket};
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
    connection: Option<UdpSocket>,
}

impl UdpClient {
    pub fn new(host: String, port: u16) -> Self {
        UdpClient { host, port, connection: None }
    }

    pub fn connect(&mut self) -> Result<(), String>{
        let host = self.host.clone();
        let port = self.port;

        let socket = UdpSocket::bind("0.0.0.0:0")
            .expect("Unable to connect to the host");

        socket.set_nonblocking(true)
            .map_err(|e| e.to_string())?;

        let address = format!("{}:{}", host, port);

        let result = socket
            .send_to("handshake".as_bytes(), address)
            .map_err(|e| e.to_string())
            .map(|_| ());

        self.connection = Some(socket);

        result
    }

    pub fn read_data(&mut self) -> Result<Vec<String>, String> {
        if self.connection.is_none() {
            self.connect().unwrap();
            return self.read_data()
        } else {
            let socket = self.connection.as_mut().unwrap();
            let mut buff = vec![0u8; 1024];

            let mut content = String::new();
            loop {
                let result = socket.recv(&mut buff);

                if result.is_err() {
                    break;
                }
                let read_cnt = result.unwrap();
                if read_cnt == 0 {
                    break;
                }

                let ct = String::from_utf8(buff[0..read_cnt].to_vec()).unwrap();
                content.push_str(&ct);
            }
            Ok(content.trim()
                .split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
        }
    }
}
