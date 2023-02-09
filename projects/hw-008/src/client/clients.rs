use std::io::{stdout, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct TcpClient {
    host: String,
    port: u16,
}

impl TcpClient {
    pub fn new(host: String, port: u16) -> Self {
        TcpClient { host, port }
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

    fn read_data(&self, socket: &mut TcpStream) -> Result<String, String> {
        let mut reader = BufReader::new(socket);
        let mut buf = [0; 4];
        reader.read_exact(&mut buf).unwrap();
        let len = u32::from_be_bytes(buf);

        let mut buf = vec![0; len as _];
        reader.read_exact(&mut buf).unwrap();
        let result = String::from_utf8(buf).map_err(|e| e.to_string())?;

        Ok(result)
    }

    fn write_data(&self, socket: &mut TcpStream, data: &[u8]) -> Result<(), String> {
        socket
            .set_write_timeout(Some(Duration::from_secs(3)))
            .map_err(|e| format!("Unable to set write timeout: {:?}", e))?;

        socket
            .write_all(data)
            .map_err(|e| format!("Error: {:?}", e))?;
        Ok(())
    }

    pub fn run(&self) -> Result<(), String> {
        let mut stream = TcpStream::connect((self.host.clone(), self.port))
            .expect("Unable to connect to the host");

        self.write_data(&mut stream, b"handshake")?;

        let handshake = self.read_data(&mut stream)?;

        if handshake.trim() != "handshake" {
            return Err(format!("Expected handshake message, but got {}", handshake));
        }

        println!(
            "Connection established.\n \
                Type --help to get detailed info. \
                Type `exit` or `quit` to exit from the app"
        );

        let mut stdin = std::io::stdin().lock();
        let mut quit = false;

        while !quit {
            let mut command = String::new();
            self.write_prompt()?;

            stdin.read_line(&mut command).expect("Unable read command");
            let command = command.trim();

            if command == "quit" || command == "exit" {
                quit = true;
                continue;
            }

            self.write_data(&mut stream, command.as_bytes())?;
            let response = self.read_data(&mut stream)?;
            self.write_to_console(&response)?;
        }

        Ok(())
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
