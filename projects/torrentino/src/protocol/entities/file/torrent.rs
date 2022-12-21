use crate::protocol::entities::file::torrent_node::TorrentNode;
use crate::protocol::entities::TorrentInfo;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde_bencode::de;
use serde_derive::Deserialize;
use sha1::{Digest, Sha1};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

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

impl Torrent {
    pub fn trackers_list(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        if let Some(tracker) = self.announce.clone() {
            result.push(tracker);
        }

        if let Some(trackers) = self.announce_list.clone() {
            for tracker in trackers.iter() {
                result.push(tracker[0].clone())
            }
        }
        result
    }

    pub fn info_hash(&self) -> Result<[u8; 20], String> {
        let info = serde_bencode::to_bytes(&self.info)
            .map_err(|e| format!("Unable to serialize torrent info {:?}", e))?;
        let digest = Sha1::digest(&info);
        let mut info_hash = [0u8; 20];
        info_hash.copy_from_slice(&digest);
        Ok(info_hash)
    }

    pub fn total_size(&self) -> u64 {
        if let Some(ref files) = self.info.files {
            files.iter().map(|f| f.length).sum()
        } else {
            0
        }
    }
}

impl TryFrom<PathBuf> for Torrent {
    type Error = String;

    fn try_from(file: PathBuf) -> Result<Self, Self::Error> {
        let mut file = File::open(file).map_err(|e| format!("Unable open file due error: {e}"))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("{}", e))?;

        let torrent = de::from_bytes::<Torrent>(&buffer)
            .map_err(|e| format!("Unable deserialize the bencode file: {e}"))?;

        Ok(torrent)
    }
}

impl TryFrom<String> for Torrent {
    type Error = String;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        let mut file = File::open(path).map_err(|e| format!("Unable open file due error: {e}"))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("{}", e))?;

        let torrent = de::from_bytes::<Torrent>(&buffer)
            .map_err(|e| format!("Unable deserialize the bencode file: {e}"))?;

        Ok(torrent)
    }
}

impl Display for Torrent {
    #[allow(unused_must_use)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        fn write_announce_list(announce_list: &Vec<Vec<String>>, formatter: &mut Formatter<'_>) {
            if !announce_list.is_empty() {
                write(
                    &format!("Tier 1: {}", &announce_list[0][0]),
                    "Announce List",
                    formatter,
                );

                for (index, tracker) in announce_list.iter().skip(1).enumerate() {
                    write(
                        &format!("Tier {}: {}", index + 2, &tracker[0]),
                        &format!("{:20}", ""),
                        formatter,
                    )
                }
            }
        }

        fn write<T: Display>(value: &T, name: &str, formatter: &mut Formatter<'_>) {
            formatter.write_str(&format!("{:20} {}\n", name, value));
        }

        fn write_option<T: Display>(value: Option<&T>, name: &str, formatter: &mut Formatter<'_>) {
            if let Some(v) = value {
                write(v, name, formatter)
            }
        }

        write(&self.info.name, "Name", formatter);
        write_option(self.comment.as_ref(), "Comment", formatter);
        write_option(
            self.creation_date
                .as_ref()
                .map(|timestamp| {
                    // Create a NaiveDateTime from the timestamp
                    let naive = NaiveDateTime::from_timestamp_opt(*timestamp, 0)
                        .expect("invalid or out-of-range datetime");

                    // Create a normal DateTime from the NaiveDateTime
                    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

                    // Format the datetime how you want
                    let newdate = datetime.format("%Y-%m-%d %H:%M:%S UTC");
                    format!("{}", newdate)
                })
                .as_ref(),
            "Creation Date",
            formatter,
        );
        write_option(self.created_by.as_ref(), "Created By", formatter);
        // write_option(self.info..as_ref(), "Info Hash", formatter);
        // write_option(self.info..as_ref(), "Torrent size", formatter);
        // write_option(self.info..as_ref(), "Content size", formatter);
        write(&self.info.private.unwrap_or_default(), "Private", formatter);
        write_option(self.announce.as_ref(), "Tracker", formatter);
        let default_announce_list = vec![Vec::new()];
        let announce_list = self
            .announce_list
            .as_ref()
            .unwrap_or(&default_announce_list);
        write_announce_list(announce_list, formatter);

        // write_option(self.info..as_ref(), "Piece Size", formatter);
        // write(self.info.pieces.as_ref(), "Piece Count", formatter);
        write(
            &self
                .info
                .files
                .as_ref()
                .map(|v| v.len())
                .unwrap_or_default(),
            "Files Count",
            formatter,
        );

        FmtResult::Ok(())
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
