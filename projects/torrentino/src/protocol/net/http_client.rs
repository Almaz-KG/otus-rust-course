use crate::protocol::entities::*;
use crate::protocol::net::{NetworkClient, Peer};

#[derive(Debug, Default)]
pub struct HttpClient {}

impl HttpClient {}

impl NetworkClient for HttpClient {
    fn get_peers_list(
        &self,
        _torrent: &Torrent,
        tracker_url: &TrackerUrl,
    ) -> Result<Vec<Peer>, String> {
        println!("Tracker url: {:?}", tracker_url);

        unimplemented!()
    }
}
