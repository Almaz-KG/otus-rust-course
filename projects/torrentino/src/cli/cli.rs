use crate::protocol::entities::Torrent;
use crate::protocol::net::UdpClient;
use crate::cli::Arguments;
use std::convert::TryFrom;
use url::{Url, ParseError};

pub struct Cli {
    args: Arguments,
}

impl Cli {
    pub fn new(args: Arguments) -> Self {
        Cli { args }
    }

    fn check_file_existence(&self) -> Result<(), String>{
        let file = self.args.file.as_path();
        if !file.exists() {
            return Err("Torrent file doesn't exists".to_string())
        }

        if !file.is_file() {
            return Err("Provided file is a directory".to_string())
        }

        Ok(())
    }

    fn parse_torrent_file(&self) -> Result<Torrent, String> {
        let file_path = self.args.file.as_os_str().to_str()
            .ok_or("Unable create file path".to_string())?;
        let file_path = file_path.to_string();

        Ok(Torrent::try_from(file_path)
            .map_err(|e| format!("Unable parse torrent file {}", e))?)
    }

    fn parse_announce(address: &str) -> Result<(String, u16), String> {
        let result = Url::parse(address)
            .map_err(|e| format!("{}", e))?;

        let host = result.host().expect("Unable extract host from announce address");
        let port = result.port().expect("Unable extract port from announce address");

        Ok((host.to_string(), port))
    }

    pub fn process(&self) -> Result<(), String> {
        self.check_file_existence()?;

        let torrent = self.parse_torrent_file()?;

        let (host, port) = Cli::parse_announce(&torrent.announce
            .expect("No announce provided in torrent file"))
            .expect("Unable extract host and port from announce address");

        let mut udp_client = UdpClient::new(34254,host, port);
        let peers_list = udp_client.get_peers_list()?;

        println!("{:?}", peers_list);

        Ok(())
    }
}