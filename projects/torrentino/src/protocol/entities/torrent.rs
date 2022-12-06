use crate::protocol::entities::torrent_node::TorrentNode;
use crate::protocol::entities::TorrentInfo;

use serde_derive::Deserialize;
use std::convert::TryFrom;
use serde_bencode::de;
use std::fs::File;
use std::io::Read;


#[derive(Debug, Deserialize)]
pub struct Torrent {
    #[serde(default)]
    pub announce: Option<String>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,
    #[serde(default)]
    pub encoding: Option<String>,
    pub info: TorrentInfo,
    #[serde(default)]
    pub nodes: Option<Vec<TorrentNode>>,
    #[serde(default)]
    pub httpseeds: Option<Vec<String>>,
}

impl TryFrom<String> for Torrent {
    type Error = String;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        let mut file = File::open(path)
            .map_err(|e| format!("Unable open file due error: {e}"))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("{}", e))?;

        let torrent = de::from_bytes::<Torrent>(&buffer)
            .map_err(|e| format!("Unable deserialize the bencode file: {e}"))?;

        Ok(torrent)
    }
}
