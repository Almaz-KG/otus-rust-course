use crate::engine::generate_peer_id;
use crate::protocol::entities::*;
use crate::protocol::net::{NetworkClient, Peer};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

const DEFAULT_BUFFER_SIZE: usize = 32767;

#[derive(Debug, Default)]
pub struct UdpClient {}

impl UdpClient {
    fn make_request(
        &self,
        request_content: &[u8],
        tracker: &TrackerUrl,
    ) -> Result<Vec<u8>, String> {
        if tracker.protocol != TrackerProtocol::UDP {
            // Skip non UDP trackers
            return Err(format!(
                "Unsupported tracker protocol: {}",
                tracker.protocol
            ));
        }

        let remote_address: SocketAddr = format!("{}:{}", tracker.url, tracker.port)
            .to_socket_addrs()
            .expect("Unable create remote host address")
            .as_slice()[0];

        // We'll bind our UDP socket to a local IP/port, but for now we basically let the OS
        // pick both of those.
        let bind_addr = if remote_address.ip().is_ipv4() {
            "0.0.0.0:0"
        } else {
            "[::]:0"
        };

        let socket = UdpSocket::bind(bind_addr).expect("Unable open UDP socket");

        socket
            .set_read_timeout(Some(Duration::from_secs(10)))
            .map_err(|e| format!("{}", e))?;

        let _ = socket
            .send_to(request_content, remote_address)
            .map_err(|e| format!("{}", e))?;
        let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];

        let (size, _) = socket
            .recv_from(&mut buffer)
            .map_err(|e| format!("{}", e))?;

        Ok(buffer[0..size].to_vec())
    }
}

impl NetworkClient for UdpClient {
    fn get_peers_list(
        &self,
        torrent: &Torrent,
        tracker_url: &TrackerUrl,
    ) -> Result<Vec<Peer>, String> {
        let request_content = bincode::serialize(&ConnectionRequest::default()).unwrap();
        let response_raw: Vec<u8> = self.make_request(&request_content, tracker_url)?;
        let connection_response: ConnectionResponse =
            bincode::deserialize(&response_raw).map_err(|e| format!("{}", e))?;

        let connection_id = connection_response.connection_id;
        println!("Connection id: {}", connection_id);

        let info_hash: [u8; 20] = torrent.info_hash()?;
        let peer_id: [u8; 20] = generate_peer_id();
        let total_size: u64 = torrent.total_size();
        // TODO: generate the port value
        let port: u16 = 6881;

        let request: AnnounceRequest =
            AnnounceRequest::announce(connection_id, info_hash, peer_id, total_size, port);

        let request_content = bincode::serialize(&request).unwrap();
        let response_raw: Vec<u8> = self.make_request(&request_content, tracker_url)?;

        let peers = Peer::from_bytes(&response_raw[20..])?;
        Ok(peers)
    }
}
