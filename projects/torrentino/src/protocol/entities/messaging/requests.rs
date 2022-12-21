use rand::random;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Eq, PartialEq)]
pub enum TrackerProtocol {
    TCP,
    UDP,
    WSS,
}

impl TrackerProtocol {
    pub fn from_url(url: &str) -> Option<Self> {
        if url.starts_with("udp") {
            Some(TrackerProtocol::UDP)
        } else if url.starts_with("wss") {
            Some(TrackerProtocol::WSS)
        } else if url.starts_with("tcp") {
            Some(TrackerProtocol::TCP)
        } else {
            None
        }
    }
}

impl Display for TrackerProtocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::UDP => formatter.write_str("udp"),
            Self::TCP => formatter.write_str("tcp"),
            Self::WSS => formatter.write_str("wss"),
        }
    }
}

impl Default for TrackerProtocol {
    fn default() -> Self {
        Self::UDP
    }
}

#[derive(Debug)]
pub struct TrackerUrl {
    pub protocol: TrackerProtocol,
    pub url: String,
    pub port: u16,
}

impl TrackerUrl {
    pub fn new(protocol: TrackerProtocol, url: String, port: u16) -> Self {
        Self {
            protocol,
            url,
            port,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionRequest {
    protocol_id: i64,
    action: i32,
    transaction_id: i32,
}

impl Default for ConnectionRequest {
    fn default() -> Self {
        Self {
            protocol_id: i64::to_be(0x41727101980),
            action: 0,
            transaction_id: random(),
        }
    }
}

impl Display for ConnectionRequest {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        formatter.write_str(&format!("{:#?}", self))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionResponse {
    pub action: i32,
    pub transaction_id: i32,
    pub connection_id: i64,
}
