use crate::protocol::entities::{Torrent, TrackerUrl};

pub trait NetworkClient {
    fn get_peers_list(
        &self,
        torrent: &Torrent,
        tracker: &TrackerUrl,
    ) -> Result<Vec<String>, String>;
}
