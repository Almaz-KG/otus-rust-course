use crate::protocol::entities::*;
use crate::protocol::net::{NetworkClient, Peer};

#[derive(Debug, Default)]
pub struct HttpClient {}

impl HttpClient {}

impl NetworkClient for HttpClient {
    fn obtain_connection_id(&self, _tracker: &TrackerUrl) -> Result<i64, String> {
        unimplemented!()
    }

    fn get_peers_list(
        &self,
        _torrent: &Torrent,
        _tracker_url: &TrackerUrl,
    ) -> Result<Vec<Peer>, String> {
        unimplemented!()
    }
}
