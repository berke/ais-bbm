use super::*;

// Ref: https://www.e-navigation.nl/content/inland-ship-static-and-voyage-related-data
// DAC 200
// FID 10

#[derive(Clone,Debug)]
pub enum HazardousCargo {
    NoBlue,
    Blue1,
    Blue2,
    Blue3,
    BFlag,
    Unknown
}

impl TryFrom<u8> for HazardousCargo {
    type Error = Error;

    fn try_from(x:u8)->Result<Self> {
        Ok(match x {
            0 => Self::NoBlue,
            1 => Self::Blue1,
            2 => Self::Blue2,
            3 => Self::Blue3,
            4 => Self::BFlag,
            _ => Self::Unknown,
        })
    }
}

#[derive(Clone,Debug)]
pub enum Loaded { 
    NotAvailable,
    Loaded,
    Unloaded,
    Invalid
}

impl TryFrom<u8> for Loaded {
    type Error = Error;

    fn try_from(x:u8)->Result<Self> {
        Ok(match x {
            0 => Self::NotAvailable,
            1 => Self::Loaded,
            2 => Self::Unloaded,
            _ => Self::Invalid,
        })
    }
}

#[derive(Clone,Debug)]
pub struct InlandShipVoyage {
    pub evin:String,
    pub ship_length:f64,
    pub ship_beam:f64,
    pub eri_type:u16,
    pub hazardous_cargo:HazardousCargo,
    pub draught:f64,
    pub loaded:Loaded,
    pub speed_quality_high:bool,
    pub course_quality_high:bool,
    pub heading_quality_high:bool,
    pub spare:u8
}

impl InlandShipVoyage {
    pub fn parse(msg:&BinaryBroadcastMessage)->Result<Self> {
        let mut u = Bits::from(&msg.data[..]);

        let mut evin = String::new();
        for _ in 0..8 {
            let c = u.bits::<u8>(6)?;
            evin.push(itu_to_char(c));
        }
	let ship_length = to_range(0.1,1,8000,u.bits::<u32>(13)?);
	let ship_beam = to_range(0.1,1,1000,u.bits::<u32>(10)?);
        let eri_type = u.bits::<u16>(14)?;
        let hazardous_cargo = u.bits::<u8>(3)?.try_into()?;
	let draught = to_range(0.1,1,2000,u.bits::<u32>(11)?);
        let loaded = u.bits::<u8>(2)?.try_into()?;
        let speed_quality_high = u.bit()?;
        let course_quality_high = u.bit()?;
        let heading_quality_high = u.bit()?;
        let spare = u.bits::<u8>(8)?;
        if !u.is_empty() {
            bail!("Trailing bits");
        }
        Ok(Self {
            evin,
            ship_length,
            ship_beam,
            eri_type,
            hazardous_cargo,
            draught,
            loaded,
            speed_quality_high,
            course_quality_high,
            heading_quality_high,
            spare,
	})
    }
}
