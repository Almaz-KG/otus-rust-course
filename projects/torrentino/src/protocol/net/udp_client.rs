use crate::protocol::entities::AnnounceResponse;
use crate::protocol::entities::ConnectionRequest;
use crate::protocol::entities::ConnectionType;
use std::io;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

#[derive(Debug)]
pub struct UdpClient {
    local_port: u16,

    remote_host: String,
    remote_port: u16,

    socket: Option<UdpSocket>,
}

impl UdpClient {
    pub fn new(local_port: u16, remote_host: String, remote_port: u16) -> Self {
        Self {
            local_port,
            remote_host,
            remote_port,
            socket: None,
        }
    }

    pub fn get_peers_list(&mut self) -> Result<Vec<String>, String> {
        let connection_id = self.establish_connection()?;
        let peers = self.make_announce_request(&connection_id)?;

        // Send a connect request
        // Get the connect response and extract the connection id
        // Use the connection id to send an announce request - this is where we tell the tracker which files we’re interested in
        // Get the announce response and extract the peers list

        todo!()
    }

    fn init_socket(&mut self) {
        let host_address: SocketAddr = format!("127.0.0.1:{}", self.local_port)
            .to_socket_addrs()
            .unwrap()
            .as_slice()[0];

        // We'll bind our UDP socket to a local IP/port, but for now we basically let the OS
        // pick both of those.
        let bind_addr = if host_address.ip().is_ipv4() {
            "0.0.0.0:0"
        } else {
            "[::]:0"
        };

        let socket = UdpSocket::bind(&bind_addr).expect("Unable open UDP socket");

        let remote_address = format!("{}:{}", self.remote_host, self.remote_port);
        println!("{}", remote_address);
        socket.connect(remote_address);
        socket.set_read_timeout(Some(Duration::from_secs(5)));

        self.socket = Some(socket);
    }

    fn establish_connection(&mut self) -> Result<ConnectionType, String> {
        if self.socket.is_none() {
            self.init_socket();
        }

        let socket = self.socket.as_ref().unwrap();

        let cr = ConnectionRequest::new();
        let cr_code = bincode::serialize(&cr).unwrap();

        println!("{:?}", &cr_code);

        let send = match socket.send(&cr_code) {
            io::Result::Err(e) => Err(format!("{}", e)),
            io::Result::Ok(size) => Ok(size),
        }?;
        println!("{}", send);

        let mut buffer = [0u8; 16];
        socket
            .recv_from(&mut buffer)
            .expect("Could not read into buffer");

        // let (number_of_bytes, src_addr) = socket
        //     .recv_from(&mut buff)
        //     .expect("no data received");
        //
        // println!("{:?}", number_of_bytes);
        println!("{:?}", buffer);

        todo!()
    }

    fn make_announce_request(&self, connection_id: &str) -> Result<AnnounceResponse, String> {
        todo!()
    }
}
