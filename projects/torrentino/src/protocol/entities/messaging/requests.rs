use rand::random;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionRequest {
    protocol_id: i64,
    action: i32,
    transaction_id: i32,
}

impl ConnectionRequest {
    pub fn new() -> Self {
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
