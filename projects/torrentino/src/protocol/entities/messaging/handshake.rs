use bytes::{Bytes, BytesMut};

pub struct HandshakeRequest {
    info_hash: [u8; 20],
    peer_id: [u8; 20],
}

impl HandshakeRequest {
    pub fn create(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        HandshakeRequest {
            info_hash,
            peer_id
        }
    }

    pub fn as_bytes(&self) -> Bytes {
        let mut handshake = BytesMut::with_capacity(68);
        handshake.extend_from_slice(&[
            19, // pstrlen. Always 19 in the 1.0 protocol
            66, 105, 116, 84, 111, 114, 114, 101, 110, 116, 32, 112, 114, 111, 116, 111, 99, 111,
            108, // pstr. Always "BitTorrent protocol" in the 1.0 protocol
        ]);
        handshake.extend_from_slice(&[0u8; 8]); // Reserved 8 bytes
        handshake.extend_from_slice(&self.info_hash);
        handshake.extend_from_slice(&self.peer_id);
        handshake.freeze()
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::entities::HandshakeRequest;

    #[test]
    fn build_default_and_serialize(){
        let peer_id = [0u8; 20];
        let info_hash = [0u8; 20];
        let handshake = HandshakeRequest::create(info_hash, peer_id);

        let request_content = handshake.as_bytes();
        assert_eq!(request_content.len(), 68);


    }
}