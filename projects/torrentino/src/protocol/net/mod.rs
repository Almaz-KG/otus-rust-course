mod download_from_peer;
mod http_client;
mod network_client;
mod udp_client;

pub use http_client::*;
pub use network_client::*;
use std::fmt::{Display, Formatter};
pub use udp_client::*;

#[derive(Debug)]
pub struct Peer {
    pub ip: String,
    pub port: u16,
}

impl Display for Peer {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&format!("{}:{}", self.ip, self.port))
    }
}

impl Peer {
    pub fn from_bytes(bytes: &[u8]) -> Result<Vec<Peer>, String> {
        let mut peers: Vec<Peer> = vec![];
        if bytes.len() % 6 != 0 {
            return Err("Malformed byte array".to_string());
        }

        for chunk in bytes.chunks(6) {
            // Peer is u32 ip, u16 port = 6 bytes total
            let peer: Peer = Peer {
                // big endian
                ip: format!("{}.{}.{}.{}", chunk[0], chunk[1], chunk[2], chunk[3]),
                port: u16::from_ne_bytes([chunk[5], chunk[4]]),
            };
            peers.push(peer);
        }

        Ok(peers)
    }
}
