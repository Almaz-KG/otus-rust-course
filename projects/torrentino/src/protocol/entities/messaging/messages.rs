use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BufMut, Bytes, BytesMut};

/// The initial state of each peer in the swarm is the Chocked and Not Interested, which means
/// nobody wants to speak with each other.
///
/// The desired state for being able to download data from the swarm is to change the state of
/// the remote peer to Unchocked and Interested. In this case, remote peer will answer to your
/// requests
#[derive(Debug)]
pub enum PeerState {
    /// If you got this peer state, that means peer is not want to talk with you
    /// You're not allowed to sent any request to this peer, and peer will not answer for your
    /// requests
    Chocked,

    /// If you got this peer state, that means peer is ready to talk with you. Most probably, it
    /// replay for you well defined requests
    Interested,
}

/// All of the remaining messages in the protocol take the form of
/// `<length prefix><messageID><payload>`. The length prefix is a four byte big-endian value.
/// The message ID is a single decimal byte. The payload is message dependent.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum MessageType {
    /// keep-alive: <len=0000>
    ///      The keep-alive message is a message with zero bytes, specified with the length prefix
    ///      set to zero. There is no message ID and no payload. Peers may close a connection if they
    ///      receive no messages (keep-alive or any other message) for a certain period of time, so a
    ///      keep-alive message must be sent to maintain the connection alive if no command have been
    ///      sent for a given amount of time. This amount of time is generally two minutes.
    KeepAlive,

    /// choke: <len=0001><id=0>. The choke message is fixed-length and has no payload.
    Choke,

    /// unchoke: <len=0001><id=1>. The unchoke message is fixed-length and has no payload.
    Unchoke,

    /// interested: <len=0001><id=2>. The interested message is fixed-length and has no payload.
    Interested,

    /// not interested: <len=0001><id=3>. The not interested message is fixed-length and has no
    ///       payload.
    NotInterested,

    /// have: <len=0005><id=4><piece index>. The have message is fixed length. The payload is the
    ///      zero-based index of a piece that has just been successfully downloaded and verified
    ///      via the hash.
    Have(u32),

    /// bitfield: <len=0001+X><id=5><bitfield>. The bitfield message may only be sent immediately
    /// after the handshaking sequence is completed, and before any other messages are sent. It
    /// is optional, and need not be sent if a client has no pieces. The bitfield message is
    /// variable length, where X is the length of the bitfield. The payload is a bitfield
    /// representing the pieces that have been successfully downloaded. The high bit in the
    /// first byte corresponds to piece index 0. Bits that are cleared indicated a missing
    /// piece, and set bits indicate a valid and available piece. Spare bits at the end are set to zero.
    /// A bitfield of the wrong length is considered an error. Clients should drop the connection
    /// if they receive bitfields that are not of the correct size, or if the bitfield has any of
    /// the spare bits set.
    Bitfield(Vec<bool>),

    /// request: <len=0013><id=6><index><begin><length>. The request message is fixed length, and
    /// is used to request a block. The payload contains the following information:
    ///     - index: integer specifying the zero-based piece index
    ///     - begin: integer specifying the zero-based byte offset within the piece
    ///     - length: integer specifying the requested length.
    Request(u32, u32, u32),

    /// piece: <len=0009+X><id=7><index><begin><block>. The piece message is variable length,
    /// where X is the length of the block. The payload contains the following information:
    ///     - index: integer specifying the zero-based piece index
    ///     - begin: integer specifying the zero-based byte offset within the piece
    ///     - block: block of data, which is a subset of the piece specified by index.
    Piece,

    /// cancel: <len=0013><id=8><index><begin><length>. The cancel message is fixed length, and is
    /// used to cancel block requests. The payload is identical to that of the "request" message.
    /// It is typically used during "End Game"
    Cancel,
    /// port: <len=0003><id=9><listen-port>. The port message is sent by newer versions of the
    /// Mainline that implements a DHT tracker. The listen port is the port this peer's DHT node
    /// is listening on. This peer should be inserted in the local routing table
    /// (if DHT tracker is supported).
    Port,
}

impl MessageType {
    fn build_have_from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, String> {
        todo!()
    }

    fn build_bitfield_from_cursor(cursor: &mut Cursor<&[u8]>, len: u32) -> Result<Self, String> {
        let mut result = vec![false; len as usize];

        for i in 0..result.len() {
            result[i] = cursor.get_u8() == u8::MAX
        }

        Ok(MessageType::Bitfield(result))
    }

    fn build_request_from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, String> {
        todo!()
    }

    fn build_piece_from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, String> {
        todo!()
    }

    fn build_cancel_from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, String> {
        todo!()
    }

    fn build_port_from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, String> {
        todo!()
    }

    pub fn to_bytes(&self) -> Bytes {
        match self {
            MessageType::Interested => {
                let mut interested = BytesMut::with_capacity(5);
                interested.extend_from_slice(&[0, 0, 0, 0]);
                interested.extend_from_slice(&[2]);
                assert_eq!(interested.len(), 5);
                interested.freeze()
            },

            MessageType::Request(index, start, len) => {
                let mut request = BytesMut::with_capacity(17);
                request.put_u32(13); // len
                request.put_u8(6); // id
                request.put_u32(*index);
                request.put_u32(*start);
                request.put_u32(*len);
                assert_eq!(request.len(), 17);
                request.freeze()
            }
            _ => todo!()
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let mut cursor = Cursor::new(bytes);
        let length: u32 = cursor.read_u32::<BigEndian>().unwrap();
        let id: u8 = cursor.read_u8().unwrap();

        match (length, id) {
            (0, _) => Ok(Self::KeepAlive),
            (1, 0) => Ok(Self::Choke),
            (1, 1) => Ok(Self::Unchoke),
            (1, 2) => Ok(Self::Interested),
            (1, 3) => Ok(Self::NotInterested),
            (5, 4) => MessageType::build_have_from_cursor(&mut cursor),
            (len @ _, 5) => MessageType::build_bitfield_from_cursor(&mut cursor, len - 1),
            (13, 6) => MessageType::build_request_from_cursor(&mut cursor),
            (len @ _, 7) => MessageType::build_piece_from_cursor(&mut cursor),
            (13, 8) => MessageType::build_cancel_from_cursor(&mut cursor),
            (3, 9) => MessageType::build_port_from_cursor(&mut cursor),
            (_, _) => Err("Unsupported message type".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::entities::MessageType;

    #[test]
    fn test_build_interested_request() {
        let bytes = MessageType::Interested.to_bytes();

        assert_eq!(bytes.to_vec(), vec![0, 0, 0, 0, 2]);
    }

    #[test]
    fn test_bitfield_request(){
        let content = [0, 0, 0, 25, 5,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 254];

        let bitfield = MessageType::from_bytes(&content).unwrap();

        match bitfield {
            MessageType::Bitfield(bit) => {
                let len = 24;
                let mut expected = vec![true; len];
                expected[len - 1] = false;

                assert_eq!(bit.as_slice(), expected);
            },
            _ => panic!("Unexpected message type")
        }
    }

    #[test]
    fn test_request_request(){
        let bytes = MessageType::Request(1, 0, u32::MAX).to_bytes();

        println!("{:?}", bytes.to_vec());
    }
}