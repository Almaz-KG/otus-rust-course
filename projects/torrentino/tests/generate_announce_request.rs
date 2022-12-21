use std::convert::TryFrom;
use std::path::PathBuf;
use torrentino::engine::generate_peer_id;
use torrentino::protocol::entities::{AnnounceRequest, Torrent};

#[test]
fn generate_announce_request() {
    let file: PathBuf = "resources/test_file.torrent".to_string().parse().unwrap();

    let torrent = Torrent::try_from(file).expect("Unable parse torrent file");

    let connection_id: i64 = -1;
    let peer_id: [u8; 20] = generate_peer_id();

    let info_hash = torrent
        .info_hash()
        .expect("Unable calculate torrent info hash");
    let total_size = torrent.total_size();

    let port: u16 = 6891;

    let request = AnnounceRequest::announce(connection_id, info_hash, peer_id, total_size, port);

    let request_content = bincode::serialize(&request).unwrap();

    assert_eq!(request_content.len(), 98);
}
