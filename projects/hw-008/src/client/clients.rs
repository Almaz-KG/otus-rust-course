use std::io::{stdout, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use clap::builder::Str;
use serde_json::to_string;
use crate::commands::ClientCommand;
use crate::ServerResponse;

pub struct TcpClient {
    host: String,
    port: u16,

    connection: Option<TcpStream>,
}

impl TcpClient {
    pub fn new(host: String, port: u16) -> Self {
        TcpClient { host, port, connection: None }
    }

    fn write_to_console(&self, content: &str) -> Result<(), String> {
        if !content.is_empty() {
            print!("{}", content);
            if content.as_bytes()[content.len() - 1] != b'\n' {
                println!();
            }
            stdout()
                .flush()
                .map_err(|e| format!("Couldn't flush stdout: {:?}", e))?;
        }
        Ok(())
    }

    fn write_prompt(&self) -> Result<(), String> {
        print!("->");
        stdout()
            .flush()
            .map_err(|e| format!("Couldn't flush stdout: {:?}", e))?;

        Ok(())
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

        return Ok("Connection established.\n \
                Type --help to get detailed info. \
                Type `exit` or `quit` to exit from the app".to_string());
    }

    pub fn command(&mut self, command: String) -> ServerResponse {
        if self.connection.is_none() {
            self.connect().unwrap();
        }

        let mut stream = self.connection.as_mut().unwrap();

        TcpClient::write_data(&mut stream, command.as_bytes()).unwrap();
        let response = TcpClient::read_data(&mut stream)?;
        Ok(response)

        // if let Some(stream) = self.connection.as_mut() {
        //     self.write_data(stream, command.as_bytes())?;
        //     //
        // } else {
        //     Err("Unknown error".to_string())
        // }
    }

    // pub fn run(&self) -> Result<(), String> {
    //
    //
    //     let mut stdin = std::io::stdin().lock();
    //     let mut quit = false;
    //
    //     while !quit {
    //         let mut command = String::new();
    //         self.write_prompt()?;
    //
    //         stdin.read_line(&mut command).expect("Unable read command");
    //         let command = command.trim();
    //
    //         if command == "quit" || command == "exit" {
    //             quit = true;
    //             continue;
    //         }
    //
    //         self.write_data(&mut stream, command.as_bytes())?;
    //         let response = self.read_data(&mut stream)?;
    //         self.write_to_console(&response)?;
    //     }
    //
    //     Ok(())
    // }
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
