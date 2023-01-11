use std::io::{BufRead, Read, stdout, Write};
use std::net::TcpStream;
use std::time::Duration;
use clap::Parser;

struct TcpClient {
    host: String,
    port: u16,
}

impl TcpClient {
    pub fn new(host: String, port: u16) -> Self {
        TcpClient { host, port }
    }

    fn write_command_prompt(&self) -> Result<(), String> {
        print!("-> ");
        stdout().flush().map_err(|e| format!("Couldn't flush stdout: {:?}", e))?;
        Ok(())
    }

    fn read_string(&self, socket: &mut TcpStream) -> Result<String, String> {
        socket.set_read_timeout(Some(Duration::from_secs(3)))
            .map_err(|e| format!("Unable to set read timeout: {:?}", e))?;

        let mut received_data = String::new();

        socket.read_to_string(&mut received_data)
            .map_err(|e| format!("Error: {:?}", e))?;
        println!("AAA");

        Ok(received_data)
    }

    fn write_data(&self, socket: &mut TcpStream, data: &[u8]) -> Result<(), String> {
        socket.set_write_timeout(Some(Duration::from_secs(3)))
            .map_err(|e| format!("Unable to set write timeout: {:?}", e))?;

        socket.write_all(data).map_err(|e| format!("Error: {:?}", e))?;
        Ok(())
    }

    pub fn run(&self) -> Result<(), String> {
        let mut tcp_socket = TcpStream::connect((self.host.clone(), self.port))
            .expect("Unable to connect to the host");

        self.write_data(&mut tcp_socket, b"handshake")?;
        let handshake = self.read_string(&mut tcp_socket)?;

        if handshake != "handshake" {
            return Err(format!("Expected handshake message, but got {}", handshake).into());
        }

        println!("Connection established. \
                Type --help to get detailed info. \
                Type `exit` or `quit` to exit from the app");

        let mut stdin = std::io::stdin().lock();
        let mut quit = false;

        while !quit {
            let mut command = String::new();
            self.write_command_prompt()?;

            stdin.read_line(&mut command).expect("Unable read command");
            let command = command.trim();

            if command == "quit" || command == "exit" {
                quit = true;
                continue;
            }

            self.write_data(&mut tcp_socket, command.as_bytes())?;
            let response = self.read_string(&mut tcp_socket)?;
            println!("{}", response);
        }

        Ok(())
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ClientArgs {
    /// The Server host to connect
    #[arg(long, value_name = "host")]
    pub host: String,

    /// The Server port int connect
    #[arg(short = 'p', long)]
    pub port: u16,
}

fn main() {
    let args = ClientArgs::parse();

    let host = args.host;
    let port = args.port;

    let client = TcpClient::new(host, port);
    let result = client.run();
    println!("{:?}", result)
}
