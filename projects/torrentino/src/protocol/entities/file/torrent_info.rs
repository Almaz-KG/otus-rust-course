use crate::protocol::entities::TorrentFile;
use serde_bytes::ByteBuf;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentInfo {
    /// This field will contain the name of the file in case of single file torrent
    /// or the name of the directory in case of the multi-file torrent case.
    pub name: String,

    /// An optional a 32-character hexadecimal string corresponding to the MD5 sum of the file.
    /// This is not used by BitTorrent at all, but it is included by some programs for greater
    /// compatibility.
    #[serde(default)]
    pub md5sum: Option<String>,

    /// length of the file in bytes (integer) in single file torrent case
    #[serde(default)]
    pub length: Option<i64>,

    /// A list of dictionaries, one for each file. Each dictionary in this list contains the
    /// following keys:
    ///     - length: length of the file in bytes (integer)
    ///     - md5sum: An optional a 32-character hexadecimal string corresponding to the MD5 sum
    ///               of the file.
    ///     - path: a list containing one or more string elements that together represent the
    ///               path and filename. Each element in the list corresponds to either a directory
    ///               name or (in the case of the final element) the filename. For example, a the
    ///               file "dir1/dir2/file.ext" would consist of three string elements: "dir1",
    ///               "dir2", and "file.ext".
    #[serde(default)]
    pub files: Option<Vec<TorrentFile>>,

    /// string consisting of the concatenation of all 20-byte SHA1 hash values, one per piece
    /// (byte string, i.e. not urlencoded)
    pub pieces: ByteBuf,

    /// number of bytes in each piece (integer)
    #[serde(rename = "piece length")]
    pub piece_length: i64,

    /// An integer field, if it is set to "1", the client MUST publish its presence to
    /// get other peers ONLY via the trackers explicitly described in the metainfo file. If this
    /// field is set to "0" or is not present, the client may obtain peer from other means, e.g.
    /// PEX peer exchange, DHT. Here, "private" may be read as "no external peer source".
    #[serde(default)]
    pub private: Option<u8>,
}
