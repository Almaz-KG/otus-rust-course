mod http_client;
mod network_client;
mod udp_client;

pub use http_client::*;
pub use network_client::*;
pub use udp_client::*;

#[derive(Debug)]
pub struct Peer {
    ip: String,
    port: u16,
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
                ip: format!("{}.{}.{}.{}", chunk[3], chunk[2], chunk[1], chunk[0]),
                port: u16::from_ne_bytes([chunk[5], chunk[4]]),
            };
            peers.push(peer);
        }

        Ok(peers)
    }
}
