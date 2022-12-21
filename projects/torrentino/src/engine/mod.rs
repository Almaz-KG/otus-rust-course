mod engine_events;
mod torrent_engine;

use rand::distributions::Alphanumeric;
use rand::Rng;
pub use torrent_engine::TorrentEngine;

pub fn generate_peer_id() -> [u8; 20] {
    let chars: Vec<char> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();

    let mut result = [0u8; 20];

    for i in 0..result.len() {
        result[i] = chars[i] as u8;
    }

    result
}
