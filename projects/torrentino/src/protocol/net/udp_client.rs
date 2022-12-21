use crate::protocol::entities::{AnnounceResponse, Torrent, TrackerUrl, TrackerProtocol, ConnectionRequest, ConnectionResponse};
use std::io;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;
use url::Url;

const DEFAULT_PORT: u16 = 80;

type ConnectionId = i64;

#[derive(Debug)]
pub struct UdpClient {
    torrent: Torrent
}

impl UdpClient {
    pub fn new(torrent: Torrent) -> Self {
        Self { torrent }
    }

    fn parse_tracker_url(address: &str) -> Result<TrackerUrl, String> {
        let result = Url::parse(address).map_err(|e| format!("{}", e))?;
        let host = result
            .host()
            .expect("Unable extract host from announce address");
        let port = result
            .port()
            .unwrap_or(DEFAULT_PORT);

        let protocol = TrackerProtocol::from_url(address)
            .unwrap_or(TrackerProtocol::UDP);

        Ok(TrackerUrl::new(protocol, host.to_string(), port))
    }

    pub fn get_peers_list(&self) -> Result<Vec<String>, String> {
        let connection_id = self.establish_connection()?;
        let peers = self.make_announce_request(&connection_id)?;

        // Send a connect request
        // Get the connect response and extract the connection id
        // Use the connection id to send an announce request - this is where we tell the tracker which files we’re interested in
        // Get the announce response and extract the peers list

        todo!()
    }

    fn ping_pong(&self, request_content: &Vec<u8>, tracker: &str) -> Result<ConnectionId, String> {
        let tracker_url = UdpClient::parse_tracker_url(&tracker)?;

        if tracker_url.protocol != TrackerProtocol::UDP {
            // Skip non UDP trackers
            return Err(format!("Unsupported tracker protocol: {}", tracker_url.protocol));
        }

        let remote_address: SocketAddr = format!("{}:{}", tracker_url.url, tracker_url.port)
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

        let socket = UdpSocket::bind(&bind_addr).expect("Unable open UDP socket");

        socket.set_read_timeout(Some(Duration::from_secs(5)));

        let _ = socket.send_to(&request_content, remote_address)
            .map_err(|e| format!("{}", e))?;
        let mut buffer = [0u8; 16];
        let (_, _) = socket.recv_from(&mut buffer).map_err(|e| format!("{}", e))?;
        let response: ConnectionResponse = bincode::deserialize(&buffer)
            .map_err(|e| format!("{}", e))?;

        Ok(response.connection_id)
    }

    fn establish_connection(&self) -> Result<ConnectionId, String> {
        let request_content = bincode::serialize(&ConnectionRequest::new()).unwrap();

        for tracker in self.torrent.trackers_list() {
            println!("Trying for {}", tracker);
            let result = self.ping_pong(&request_content, &tracker);

            if result.is_ok() {
                println!("Connection established");
                return result
            }
        }

        Err("Unable connect to any torrent trackers".to_string())
    }

    fn make_announce_request(&self, connection_id: &ConnectionId) -> Result<AnnounceResponse, String> {
        println!("Connection ID: {}", connection_id);
        todo!()
    }
}
