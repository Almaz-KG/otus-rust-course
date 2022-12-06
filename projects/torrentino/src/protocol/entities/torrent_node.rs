use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TorrentNode(String, i64);