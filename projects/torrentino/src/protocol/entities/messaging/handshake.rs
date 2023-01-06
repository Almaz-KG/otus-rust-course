use bytes::{Bytes, BytesMut};

pub(crate) const HANDSHAKE_SIZE: usize = 68;

pub(crate) const BIT_TORRENT_PROTOCOL_STRING: &str = "BitTorrent protocol";

pub struct HandshakeRequest {
    info_hash: [u8; 20],
    peer_id: [u8; 20],
}

impl HandshakeRequest {
    pub fn create(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        HandshakeRequest { info_hash, peer_id }
    }

    pub fn as_bytes(&self) -> Bytes {
        let mut handshake = BytesMut::with_capacity(68);
        // pstrlen. Always 19 in the 1.0 protocol
        handshake.extend_from_slice(&[19]);

        let mut protocol = [0; 19];
        protocol.copy_from_slice(BIT_TORRENT_PROTOCOL_STRING.as_bytes());

        handshake.extend_from_slice(&protocol);
        handshake.extend_from_slice(&[0u8; 8]); // Reserved 8 bytes
        handshake.extend_from_slice(&self.info_hash);
        handshake.extend_from_slice(&self.peer_id);
        handshake.freeze()
    }

    pub fn is_valid_response(&self, bytes: &[u8]) -> bool {
        if bytes.len() < HANDSHAKE_SIZE {
            false
        } else {
            let protocol_len = bytes[0] as usize;
            let bittorrent = std::str::from_utf8(&bytes[1..=19]).unwrap();
            let _reserved = &bytes[20..28];
            let info_hash = &bytes[28..48];
            // let peer_id = &bytes[48..68]; // The remote peer id

            protocol_len == BIT_TORRENT_PROTOCOL_STRING.as_bytes().len()
                && bittorrent == BIT_TORRENT_PROTOCOL_STRING
                // && _reserved == [0u8; 8] // it might be different from peer to peer protocol
                && info_hash == self.info_hash
            // No need to compare the peer_id's, because each peer has it's own peer_id
            // && peer_id == self.peer_id
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::entities::{HandshakeRequest, HANDSHAKE_SIZE};

    #[test]
    fn build_default_and_serialize() {
        let peer_id = [0u8; 20];
        let info_hash = [0u8; 20];
        let handshake = HandshakeRequest::create(info_hash, peer_id);

        let request_content = handshake.as_bytes();
        assert_eq!(request_content.len(), HANDSHAKE_SIZE);
    }
}
