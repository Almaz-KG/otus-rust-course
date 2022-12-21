use crate::engine::generate_peer_id;
use crate::protocol::entities::*;
use crate::protocol::net::NetworkClient;
use serde::Deserialize;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct UdpClient {}

impl UdpClient {
    fn make_request<T>(&self, request_content: &[u8], tracker: &TrackerUrl) -> Result<T, String>
    where
        T: for<'a> Deserialize<'a>,
    {
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
            .set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| format!("{}", e))?;

        let _ = socket
            .send_to(request_content, remote_address)
            .map_err(|e| format!("{}", e))?;
        let mut buffer = [0u8; 16];
        let (_, _) = socket
            .recv_from(&mut buffer)
            .map_err(|e| format!("{}", e))?;
        let response: T = bincode::deserialize(&buffer).map_err(|e| format!("{}", e))?;

        Ok(response)
    }
}

impl NetworkClient for UdpClient {
    fn get_peers_list(
        &self,
        torrent: &Torrent,
        tracker_url: &TrackerUrl,
    ) -> Result<Vec<String>, String> {
        println!("Tracker url: {:?}", tracker_url);

        let request_content = bincode::serialize(&ConnectionRequest::default()).unwrap();

        let connection_response: ConnectionResponse =
            self.make_request(&request_content, tracker_url)?;

        let connection_id = connection_response.connection_id;
        println!("Connection id: {}", connection_id);

        let info_hash: [u8; 20] = torrent.info_hash()?;
        let peer_id: [u8; 20] = generate_peer_id();
        let total_size: i64 = torrent.total_size();
        // TODO: generate the port value
        let port: u16 = 6881;

        let request: AnnounceRequest =
            AnnounceRequest::announce(connection_id, info_hash, peer_id, total_size, port);

        let request_content = bincode::serialize(&request).unwrap();

        let peers: AnnounceResponse = self.make_request(&request_content, tracker_url)?;
        println!("{:?}", peers);

        // Send a connect request
        // Get the connect response and extract the connection id
        // Use the connection id to send an announce request - this is where we tell the tracker which files we’re interested in
        // Get the announce response and extract the peers list

        todo!()
    }
}
