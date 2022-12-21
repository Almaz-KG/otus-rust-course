#![allow(dead_code)]

use crate::protocol::entities::{Torrent, TrackerProtocol, TrackerUrl};
use crate::protocol::net::{NetworkClient, UdpClient, HttpClient, Peer};
use std::collections::HashMap;

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
                println!("Peers list: {:?}", peers_list);
                return Ok(peers_list)
            }
        }

        Ok(vec![])
    }

    fn download(&self, torrent: Torrent) {
        let peers_list_result = self.get_peers_list(&torrent);

        println!("{:?}", peers_list_result);
    }

    pub fn add_new_torrent(&mut self, torrent: Torrent) {
        // self.torrents_queue.push(torrent);

        // This code will be replaced to async function
        self.download(torrent);
    }
}
