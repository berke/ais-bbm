pub mod bits;
pub mod environmental;
pub mod inland_ship_voyage;
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
use inland_ship_voyage::*;
use utils::*;

pub enum AisBbm {
    Environmental(environmental::Environmental),
    MeteoHydro(meteo_hydro::MeteoHydro),
    InlandShipVoyage(inland_ship_voyage::InlandShipVoyage),
    Unhandled
}

impl AisBbm {
    pub fn parse(msg:&BinaryBroadcastMessage)->Result<Self> {
        match (msg.dac,msg.fid) {
            (1,26) => Ok(Self::Environmental(Environmental::parse(msg)?)),
            (1,31) => Ok(Self::MeteoHydro(MeteoHydro::parse(msg)?)),
            (200,10) => Ok(Self::InlandShipVoyage(InlandShipVoyage::parse(msg)?)),
            _ => Ok(Self::Unhandled)
        }
    }
}
