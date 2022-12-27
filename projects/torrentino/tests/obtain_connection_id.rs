use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;
use clap::builder::Str;
use torrentino::protocol::entities::{ConnectionRequest, ConnectionResponse, TrackerProtocol, TrackerUrl};

const DEFAULT_BUFFER_SIZE: usize = 1024;

#[test]
fn test(){
    obtain_connection_id();
}


fn obtain_connection_id() -> Result<i64, String> {
    // read tracker url info from .torrent file. See, previous section
    let tracker: TrackerUrl = TrackerUrl{
        protocol: TrackerProtocol::UDP,
        // url: "explodie.org".to_string(),
        url: "localhost".to_string(),
        port: 6969,
    };

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
    // assert_eq!(request.protocol_id, 4497486125440);

    let connection_id = response.connection_id;
    println!("{}", connection_id);

    Ok(connection_id)
}