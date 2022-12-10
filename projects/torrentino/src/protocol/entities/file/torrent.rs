use crate::protocol::entities::file::torrent_node::TorrentNode;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_torrent_file() {
        let file_path = "resources/test_file.torrent".to_string();
        let torrent = Torrent::try_from(file_path)
            .map_err(|e| format!("Unable parse torrent file {e}"))
            .unwrap();

        println!("name:\t\t{}", torrent.info.name);
        println!("announce:\t{:?}", torrent.announce);
        println!("nodes:\t\t{:?}", torrent.nodes);
        if let Some(al) = &torrent.announce_list {
            for a in al {
                println!("announce list:\t{}", a[0]);
            }
        }
        println!("httpseeds:\t{:?}", torrent.httpseeds);
        println!("creation date:\t{:?}", torrent.creation_date);
        println!("comment:\t{:?}", torrent.comment);
        println!("created by:\t{:?}", torrent.created_by);
        println!("encoding:\t{:?}", torrent.encoding);
        println!("piece length:\t{:?}", torrent.info.piece_length);
        println!("private:\t{:?}", torrent.info.private);
        println!("root hash:\t{:?}", torrent.info.root_hash);
        println!("md5sum:\t\t{:?}", torrent.info.md5sum);
        println!("path:\t\t{:?}", torrent.info.path);

        if let Some(files) = &torrent.info.files {
            for f in files {
                println!("file path:\t{:?}", f.path);
                println!("file length:\t{}", f.length);
                println!("file md5sum:\t{:?}", f.md5sum);
            }
        }
    }
}
