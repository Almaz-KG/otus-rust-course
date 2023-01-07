//! ### A super simple Tcp Smart Home Protocol
//!
//! This is a super simple tcp smart home protocol to be able to interact with clients.
//!
//! SmartHomeServer-----------------Client
//! -------------------------------Connect
//! <----------------------------Handshake
//! Handshake---------------------------->
//! --------------Repeat------------------
//! [WaitForCommand-----------------------
//! <------------------------------Command
//! CommandReply------------------------>]
//! <---------------------------------exit
//! CloseConnection---------------------->

use crate::cli::{Arguments as CliArguments, Command, CommandHandler};
use anyhow::{anyhow, Result};
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;

pub const DEFAULT_READ_WRITE_TIMEOUT_IN_SECS: Duration = Duration::from_secs(120);

#[derive(Debug, Eq, PartialEq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Handshaked,
    WaitingForCommand,
    Error(String),
}

impl Display for ConnectionStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionStatus::Connected => formatter.write_str("Connected"),
            ConnectionStatus::Disconnected => formatter.write_str("Disconnected"),
            ConnectionStatus::Handshaked => formatter.write_str("Handshaked"),
            ConnectionStatus::WaitingForCommand => formatter.write_str("WaitingForCommand"),
            ConnectionStatus::Error(msg) => formatter.write_str(&format!("Error: {}", msg)),
        }
    }
}

pub struct TcpSession {
    stream: TcpStream,
    status: ConnectionStatus,
}

impl TcpSession {
    fn print_state(&self) {
        println!("[Session][Status] {}", self.status);
    }

    fn read_line(&mut self) -> Result<String> {
        let mut buf = [0u8; 1024];

        match self.stream.read(&mut buf) {
            Ok(size) => {
                // need to delete new-line symbol at the end of the line
                let last_index = if (buf[size - 1] as char) == '\n' {
                    size - 1
                } else {
                    size
                };
                Ok(String::from_utf8(buf[0..last_index].to_vec()).unwrap())
            }
            Err(e) => Err(anyhow!("Unable read data from client: {:?}", e)),
        }
    }

    fn read_command(&mut self) -> Result<Vec<String>> {
        let mut buf = [0u8; 1024];

        match self.stream.read(&mut buf) {
            Ok(size) => {
                // need to delete new-line symbol at the end of the line
                let last_index = if (buf[size - 1] as char) == '\n' {
                    size - 1
                } else {
                    size
                };

                let args = String::from_utf8(buf[0..last_index].to_vec()).unwrap();
                let commands: Vec<String> = args.split(' ').map(|s| s.to_string()).collect();
                Ok(commands)
            }
            Err(e) => Err(anyhow!("Unable read data from client: {:?}", e)),
        }
    }

    fn write_data(&mut self, message: &str) {
        self.stream.write_all(message.as_bytes()).unwrap()
    }

    fn close_connection(&mut self) {
        self.write_data("GoodBye\n");
        self.stream.shutdown(Shutdown::Both).unwrap();
        self.status = ConnectionStatus::Disconnected;
    }

    fn make_handshake(&mut self) {
        let line = self.read_line();

        let status = match line {
            Ok(line) => {
                if line == "handshake" {
                    self.write_data("handshake\n");
                    ConnectionStatus::Handshaked
                } else {
                    ConnectionStatus::Error("handshake was expected".into())
                }
            }
            Err(msg) => ConnectionStatus::Error(format!("Error in handshake step {}", msg)),
        };

        self.status = status;
    }

    fn handle_command(&mut self, command: Vec<String>) -> bool {
        match &command[..] {
            [a] if (a.to_lowercase() == "exit" || a.to_lowercase() == "quit") => {
                self.close_connection();
                return true;
            }
            _ => {
                // FIXME: A dirty hack for clap crate. The first arg in args should be the script
                // FIXME: name. So, in our case we should give some fake script name
                let mut command_args = command.clone();
                command_args.insert(0, "/fake_script_name".into());

                let args = CliArguments::try_parse_from(command_args);

                match args {
                    Ok(args) => match &args.command {
                        Command::Init => self.write_data("Not supported command in remote mode\n"),
                        Command::Server(_) => {
                            self.write_data("Not supported command in remote mode\n")
                        }
                        _ => {
                            let output = &mut self.stream;
                            let mut handler = CommandHandler::new(output);
                            handler.process(args.command);
                        }
                    },
                    Err(e) => {
                        let error_message = e.render().to_string();
                        self.write_data(&error_message);
                    }
                }
            }
        }

        false
    }

    pub fn run(stream: TcpStream) -> Result<()> {
        stream.set_read_timeout(Some(DEFAULT_READ_WRITE_TIMEOUT_IN_SECS))?;
        stream.set_write_timeout(Some(DEFAULT_READ_WRITE_TIMEOUT_IN_SECS))?;

        let mut session = TcpSession {
            stream,
            status: ConnectionStatus::Connected,
        };

        session.print_state();
        session.make_handshake();
        session.print_state();

        if session.status != ConnectionStatus::Handshaked {
            session.write_data("No handshake\n");
            session.close_connection();
            return Err(anyhow!("No handshake"));
        }

        session.status = ConnectionStatus::WaitingForCommand;
        session.print_state();

        let mut exit = false;
        while !exit {
            match session.read_command() {
                Ok(command) => {
                    exit = session.handle_command(command);
                }
                Err(msg) => {
                    eprintln!("Unable read command: {}", msg);
                    return Err(anyhow!(format!("Unable read command: {}", msg)));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::Arguments as CliArguments;
    use clap::Parser;

    #[test]
    fn parse_client_command() {
        let command = vec!["help"];
        match CliArguments::try_parse_from(command) {
            Ok(_) => {}
            Err(e) => {
                let error_message = e.render().to_string();
                println!("{}", error_message);
            }
        }
    }

    #[test]
    fn parse_client_list_command() {
        let command = vec![" ", "list", "--homes"];
        let result = CliArguments::try_parse_from(command);
        println!("{}", result.is_ok());

        match result {
            Ok(_) => {}
            Err(e) => {
                let error_message = e.render().to_string();
                eprintln!("{}", error_message);
            }
        }
    }
}
