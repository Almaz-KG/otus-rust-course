use rand::random;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use url::Url;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum TrackerProtocol {
    HTTP,
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
        } else if url.starts_with("http") {
            Some(TrackerProtocol::HTTP)
        } else {
            None
        }
    }

    pub fn default_port(&self) -> u16 {
        match self {
            TrackerProtocol::UDP => 6891,
            TrackerProtocol::TCP => 8080,
            TrackerProtocol::HTTP => 80,
            TrackerProtocol::WSS => 80,
        }
    }
}

impl Display for TrackerProtocol {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::UDP => formatter.write_str("udp"),
            Self::TCP => formatter.write_str("tcp"),
            Self::WSS => formatter.write_str("wss"),
            Self::HTTP => formatter.write_str("http"),
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

impl TryFrom<&str> for TrackerUrl {
    type Error = String;

    fn try_from(address: &str) -> Result<Self, Self::Error> {
        let result = Url::parse(address).map_err(|e| format!("{}", e))?;
        let host = result
            .host()
            .expect("Unable extract host from announce address");

        let protocol = TrackerProtocol::from_url(address)
            .expect("Unable get tracker communication protocol");

        let port = result.port().unwrap_or_else(|| protocol.default_port());

        Ok(TrackerUrl::new(protocol, host.to_string(), port))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionRequest {
    protocol_id: u64,
    action: u32,
    transaction_id: u32,
}

impl Default for ConnectionRequest {
    fn default() -> Self {
        Self {
            protocol_id: u64::to_be(0x41727101980),
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
    pub action: u32,
    pub transaction_id: u32,
    pub connection_id: u64,
}
