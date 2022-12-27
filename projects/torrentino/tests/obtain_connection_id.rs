use torrentino::protocol::entities::{TrackerProtocol, TrackerUrl};
use torrentino::protocol::net::{NetworkClient, UdpClient};

#[test]
fn obtain_connection_id() {
    // read tracker url info from .torrent file. See, previous section
    let tracker: TrackerUrl = TrackerUrl{
        protocol: TrackerProtocol::UDP,
        url: "localhost".to_string(),
        port: 6969,
    };

    let client = UdpClient::default();
    client.obtain_connection_id(&tracker).expect("Unable establish connection");
}