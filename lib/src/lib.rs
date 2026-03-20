pub mod bits;
pub mod environmental;

use ais::{
    messages::{
        binary_broadcast_message::BinaryBroadcastMessage
    }
};
use anyhow::{
    bail,
    Result,
};

use bits::Bits;
use environmental::*;

pub enum AisBbm {
    Environmental(environmental::Environmental),
    Unhandled
}

impl AisBbm {
    pub fn parse(msg:&BinaryBroadcastMessage)->Result<Self> {
        match (msg.dac,msg.fid) {
            (1,26) => Ok(Self::Environmental(Environmental::parse(msg)?)),
            _ => Ok(Self::Unhandled)
        }
    }
}
