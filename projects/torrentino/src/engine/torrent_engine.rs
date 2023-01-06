#![allow(dead_code)]

use crate::engine::generate_peer_id;
use crate::protocol::entities::{
    HandshakeRequest, MessageType, Torrent, TrackerProtocol, TrackerUrl, HANDSHAKE_SIZE,
};
use crate::protocol::net::{HttpClient, NetworkClient, Peer, UdpClient};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub struct TorrentEngine {
    is_active: bool,
    torrents_queue: Vec<Torrent>,
    network_clients: HashMap<TrackerProtocol, Box<dyn NetworkClient>>,
}

impl TorrentEngine {
    pub fn start() -> Self {
        let mut network_clients: HashMap<TrackerProtocol, Box<dyn NetworkClient>> = HashMap::new();

        network_clients.insert(TrackerProtocol::UDP, Box::new(UdpClient::default()));
        network_clients.insert(TrackerProtocol::HTTP, Box::new(HttpClient::default()));

        TorrentEngine {
            is_active: true,
            torrents_queue: vec![],
            network_clients,
        }
    }

    /// *Protocol overview*
    /// Once a tcp connection is established the messages you send and receive have to follow the
    /// following protocol.
    ///
    /// The first thing you want to do is let your peer know which files you are interested
    /// in downloading from them, as well as some identifying info. If the peer doesn’t have the
    /// files you want they will close the connection, but if they do have the files they should
    /// send back a similar message as confirmation. This is called the “handshake”.
    ///
    /// The most likely thing that will happen next is that the peer will let you know what
    /// pieces they have. This happens through the “have” and “bitfield” messages. Each “have”
    /// message contains a piece index as its payload. This means you will receive multiple
    /// have messages, one for each piece that your peer has.
    ///
    /// The bitfield message serves a similar purpose, but does it in a different way. The
    /// bitfield message can tell you all the pieces that the peer has in just one message. It does
    /// this by sending a string of bits, one for each piece in the file. The index of each bit is the
    /// same as the piece index, and if they have that piece it will be set to 1, if not it will be set to 0.
    /// For example if you receive a bitfield that starts with 011001… that means they have the pieces at
    /// index 1, 2, and 5, but not the pieces at index 0, 3,and 4.
    ///
    /// It’s possible to receive both “have” messages and a bitfield message, if which case you
    /// should combine them to get the full list of pieces.
    ///
    /// Actually it’s possible to recieve another kind of message, the peer might decide they
    /// don’t want to share with you! That’s what the choke, unchoke, interested, and not interested
    /// messages are for. If you are choked, that means the peer does not want to share with you, if
    /// you are unchoked then the peer is willing to share. On the other hand, interested means you want
    /// what your peer has, whereas not interested means you don’t want what they have.
    ///
    /// You always start out choked and not interested. So the first message you send should be
    /// the interested message. Then hopefully they will send you an unchoke message and you can move
    /// to the next step. If you receive a choke message message instead you can just let the connection drop.
    ///
    /// Finally you will receive a piece message, which will contain the bytes of data that you
    /// requested.

    fn download_portions(
        &self,
        _peer_id: &[u8; 20],
        torrent: &Torrent,
        stream: &mut TcpStream,
        bitfield: Vec<bool>,
    ) -> Result<(), String> {
        println!("Pieces {:?}", torrent.info.piece_length);

        // if bitfield.len() != torrent.info.piece_length as usize {
        //     dbg!(bitfield.len());
        //     dbg!(torrent.info.piece_length);
        //     println!("The pieces length is not equal")
        // } else {
        for (index, have) in bitfield.iter().enumerate() {
            if *have {
                println!("Processing... {}", index);

                let request = MessageType::Request(index as u32, 0, u32::MAX);
                stream
                    .write_all(&request.to_bytes())
                    .map_err(|_| "Unable send request".to_string())?;

                let mut buff = [0u8; 1024];
                let bytes_read_cnt = stream
                    .read(&mut buff)
                    .map_err(|e| format!("Unable to read from the peer: {}", e))?;

                println!("Response: {}", bytes_read_cnt);
                println!("Content: {:?}", &buff[0..bytes_read_cnt]);
            } else {
                println!("Peer doesn't have {} piece", index)
            }
        }
        // }

        Ok(())
    }

    fn download_from_peer(
        &self,
        peer_id: &[u8; 20],
        torrent: &Torrent,
        peer: &Peer,
    ) -> Result<(), String> {
        println!("Connecting with {}", peer);
        let addr: SocketAddr = format!("{}", peer)
            .parse()
            .map_err(|e| format!("Unable create Socket address {}", e))?;

        let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(2))
            .map_err(|e| format!("Unable open TCP connection to host {}", e))?;

        let info_hash: [u8; 20] = torrent.info_hash()?;
        // make handshake
        let handshake = HandshakeRequest::create(info_hash, *peer_id);

        stream
            .write_all(&handshake.as_bytes())
            .map_err(|e| format!("Unable to write to TCP connection: {}", e))?;

        let mut buff = [0u8; 1024];
        let bytes_read_cnt = stream
            .read(&mut buff)
            .map_err(|e| format!("Unable to read from TCP connection: {}", e))?;

        if bytes_read_cnt < HANDSHAKE_SIZE {
            Err("Unexpected response length from peer".to_string())
        } else if !handshake.is_valid_response(&buff[0..HANDSHAKE_SIZE]) {
            Err("Invalid response from peer".to_string())
        } else {
            if bytes_read_cnt >= HANDSHAKE_SIZE {
                // make interest request
                let interested_request = MessageType::Interested.to_bytes();

                stream
                    .write_all(&interested_request)
                    .map_err(|e| format!("Unable write interested request to the peer {}", e))?;

                let bytes_read_cnt = stream
                    .read(&mut buff)
                    .map_err(|e| format!("Unable to read from the peer: {}", e))?;

                let response = &buff[0..bytes_read_cnt];
                let message_type = MessageType::from_bytes(response)?;
                match message_type {
                    MessageType::Have(_) => {}
                    MessageType::Bitfield(portions) => {
                        self.download_portions(peer_id, torrent, &mut stream, portions)?;
                    }
                    _ => {}
                }
            }
            todo!()
        }
    }

    fn download_from_peers(&self, torrent: &Torrent, peers: &Vec<Peer>) -> Result<(), String> {
        println!("Start downloading torrent content from peers");
        assert!(!peers.is_empty());

        let peer_id = generate_peer_id();
        println!("Main peer: {:?}", String::from_utf8(peer_id.to_vec()));

        for peer in peers.iter() {
            match self.download_from_peer(&peer_id, torrent, peer) {
                Err(msg) => println!("{}", msg),
                _ => return Ok(()),
            }
        }

        Ok(())
    }

    fn get_peers_list(&self, torrent: &Torrent) -> Result<Vec<Peer>, String> {
        for tracker in torrent.trackers_list() {
            println!("Trying for {}", tracker);
            let tracker_url = TrackerUrl::try_from(tracker.as_str());
            if tracker_url.is_err() {
                println!("Unable extract tracker_url from: {}", tracker);
                continue;
            }

            let tracker_url = tracker_url.unwrap();

            let client = self.network_clients.get(&tracker_url.protocol);
            if client.is_none() {
                println!("No client for protocol {}", tracker_url.protocol);
                continue;
            }

            let client = client.unwrap();

            if let Ok(peers_list) = client.get_peers_list(torrent, &tracker_url) {
                println!("# of peers {}", peers_list.len());
                return Ok(peers_list);
            }
        }

        // Hopefully will be never reached
        Ok(vec![])
    }

    fn download(&self, torrent: Torrent) -> Result<(), String> {
        println!("Getting peers list");
        let peers_list_result = self.get_peers_list(&torrent)?;

        self.download_from_peers(&torrent, &peers_list_result)
    }

    pub fn add_new_torrent(&mut self, torrent: Torrent) {
        // self.torrents_queue.push(torrent);

        // This code will be replaced to async function
        self.download(torrent).expect("Download failed")
    }
}
