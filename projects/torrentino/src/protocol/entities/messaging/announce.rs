use rand::random;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnounceRequest {
    connection_id: i64,
    action: u32,
    transaction_id: u32,
    info_hash: [u8; 20],
    peer_id: [u8; 20],
    downloaded: u64,
    left: u64,
    uploaded: u64,
    event: u32,
    ip_address: u32,
    key: u32,
    num_want: u32,
    port: u16,
}

impl AnnounceRequest {
    pub fn announce(
        connection_id: i64,
        info_hash: [u8; 20],
        peer_id: [u8; 20],
        total_size: u64,
        port: u16,
    ) -> Self {
        assert_eq!(info_hash.len(), 20);
        assert_eq!(peer_id.len(), 20);

        AnnounceRequest {
            connection_id,
            action: u32::to_be(1), // announce action by spec
            transaction_id: random(),
            info_hash,
            peer_id,
            downloaded: 0,
            left: total_size,
            uploaded: 0,
            event: 0, // 0: none; 1: completed; 2: started; 3: stopped,
            ip_address: 0,
            key: random(),
            num_want: u32::to_be(200),
            port,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnounceResponse {
    action: u32,
    transaction_id: u32,
    interval: u32,
    leechers: u32,
    seeders: u32,
}
