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
    fn obtain_connection_id(&self, tracker: &TrackerUrl) -> Result<i64, String> {
        // generating a default connection request structure
        let request = ConnectionRequest::default();
        // convert request body to binary array
        let request_content = bincode::serialize(&request).unwrap();

        // create a socket address to the tracker
        let remote_address: SocketAddr = format!("{}:{}", tracker.url, tracker.port)
            .to_socket_addrs()
            .expect("Unable create remote host address")
            .as_slice()[0];

        // We'll bind our UDP socket to a local IP/port,
        // but for now we basically let the OS pick both of those.
        let bind_addr = match remote_address.ip().is_ipv4() {
            false => "[::]:0", // support for ipv6
            _ => "0.0.0.0:0",
        };

        // Open an udp socket
        let socket = UdpSocket::bind(bind_addr).expect("Unable open UDP socket");

        // set timeout for the udp protocol, as udp is `unreliable` protocol
        socket
            .set_read_timeout(Some(Duration::from_secs(10)))
            .map_err(|e| format!("{}", e))?;

        // send the request
        let send_bytes = socket
            .send_to(&request_content, remote_address)
            .map_err(|e| format!("{}", e))?;

        // make sure the number of bytes sent is the same as the number of bytes in request body
        assert_eq!(send_bytes, request_content.len());

        let mut buffer = [0u8; DEFAULT_BUFFER_SIZE];

        // read response from the Tracker to buffer
        let (size, _) = socket
            .recv_from(&mut buffer)
            .map_err(|e| format!("{}", e))?;

        let response_content = &buffer[0..size];

        // deserialize the response content into Rust struct
        let response: ConnectionResponse = bincode::deserialize(response_content)
            .map_err(|e| format!("{}", e))?;

        assert_eq!(request.transaction_id, response.transaction_id);
        assert_eq!(request.action, response.action);
        // IDK why, but all torrent tracker never return back this magic number
        // assert_eq!(request.protocol_id, 4497486125440);

        let connection_id = response.connection_id;
        dbg!(connection_id);

        Ok(connection_id)
    }

    fn get_peers_list(&self, torrent: &Torrent, tracker_url: &TrackerUrl)
        -> Result<Vec<Peer>, String> {
        let connection_id = self.obtain_connection_id(tracker_url)?;

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
