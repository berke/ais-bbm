pub mod bits;
pub mod environmental;
pub mod meteo_hydro;
mod utils;

use ais::{
    messages::{
        binary_broadcast_message::BinaryBroadcastMessage,
    }
};
use anyhow::{
    bail,
    Error,
    Result,
};

use bits::Bits;
use environmental::*;
use meteo_hydro::*;
use utils::*;

pub enum AisBbm {
    Environmental(environmental::Environmental),
    MeteoHydro(meteo_hydro::MeteoHydro),
    Unhandled
}

impl AisBbm {
    pub fn parse(msg:&BinaryBroadcastMessage)->Result<Self> {
        match (msg.dac,msg.fid) {
            (1,26) => Ok(Self::Environmental(Environmental::parse(msg)?)),
            (1,31) => Ok(Self::MeteoHydro(MeteoHydro::parse(msg)?)),
            _ => Ok(Self::Unhandled)
        }
    }
}
