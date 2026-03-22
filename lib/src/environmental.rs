use super::*;

// Ref: https://www.e-navigation.nl/content/environmental
// DAC 1
// FID 26

#[derive(Clone,Debug)]
pub struct Environmental {
    pub reports:Vec<SensorReport>
}

#[derive(Clone,Debug)]
pub struct SensorReport {
    pub report_type:u8,
    pub day:u8,
    pub hour:u8,
    pub minute:u8,
    pub site_id:u8,
    pub sensor_data:SensorData
}

#[derive(Clone,Debug)]
pub enum SensorOwner {
    Unknown,
    HydrographicOffice,
    InlandWaterwayAuthority,
    CoastalDirectorate,
    MeteorologicalService,
    PortAuthority,
    CoastGuard,
    Reserved(u8)
}

#[derive(Clone,Debug)]
pub struct SiteLocation {
    pub longitude:f64,
    pub latitude:f64,
    pub altitude:f64,
    pub owner:u8, // SensorOwner,
    pub data_timeout:u8 // f32,
}

#[derive(Clone,Debug)]
pub struct StationId {
    pub name:String
}

#[derive(Clone,Debug)]
pub struct AirGap {
    pub air_draught:f64,
    pub air_gap:f64,
    pub air_gap_trend:u8,
    pub fc_air_gap:f64,
    pub fc_day:u8,
    pub fc_hour:u8,
    pub fc_minute:u8,
}

#[derive(Clone,Debug)]
pub enum SensorData {
    SiteLocation(SiteLocation),
    StationId(StationId),
    AirGap(AirGap),
    Other([u64;2])
}

const REPORT_SIZE : usize = 112;

//  0 Site Location
//  1 Station ID
// 10 Air gap / air draft

impl Environmental {
    pub fn parse(msg:&BinaryBroadcastMessage)->Result<Self> {
        let mut u = Bits::from(&msg.data[..]);
        let mut nbit = u.len();
        let mut reports = Vec::with_capacity(8);
        // let _spare = u.bits::<u8>(0)?;
        while REPORT_SIZE <= nbit {
            let report_type = u.bits::<u8>(4)?;
            let day = u.bits::<u8>(5)?;
            let hour = u.bits::<u8>(5)?;
            let minute = u.bits::<u8>(6)?;
            let site_id = u.bits::<u8>(7)?;
            // 112 - 4 = 108 = 64 + 44
            // 
            let sensor_data =
                match report_type {
                    0 => {
                        // Site location
                        let longitude = to_ll(28,1e4,u.bits::<u32>(28)?);
                        let latitude = to_ll(27,1e4,u.bits::<u32>(27)?);
                        let altitude = to_range(0.1,0,2001,u.bits::<u32>(11)?);
                        let owner = u.bits::<u8>(4)?;
                        let data_timeout = u.bits::<u8>(3)?;
                        let _spare = u.bits::<u32>(12)?;
                        SensorData::SiteLocation(SiteLocation {
                            longitude,
                            latitude,
                            altitude,
                            owner,
                            data_timeout
                        })
                    },
                    1 => {
                        // Station ID
                        let mut name = String::new();
                        for _ in 0..14 {
                            let c = u.bits::<u8>(6)?;
                            name.push(itu_to_char(c));
                        }
                        let _spare = u.bit();
                        SensorData::StationId(StationId {
                            name
                        })
                    },
                    10 => {
                        // Air gap
                        let air_draught = to_range(0.01,1,8191,u.bits::<u32>(13)?);
                        let air_gap = to_range(0.01,1,8191,u.bits::<u32>(13)?);
                        let air_gap_trend = u.bits::<u8>(2)?;
                        let fc_air_gap = to_range(0.01,1,8191,u.bits::<u32>(13)?);
                        let fc_day = u.bits::<u8>(5)?;
                        let fc_hour = u.bits::<u8>(5)?;
                        let fc_minute = u.bits::<u8>(6)?;
                        SensorData::AirGap(AirGap {
                            air_draught,
                            air_gap,
                            air_gap_trend,
                            fc_air_gap,
                            fc_day,
                            fc_hour,
                            fc_minute
                        })
                    },
                    _ => {
                        let x0 = u.bits::<u64>(64)?;
                        let x1 = u.bits::<u64>(21)?;
                        SensorData::Other([x0,x1])
                    }
                };
            reports.push(SensorReport {
                report_type,
                day,
                hour,
                minute,
                site_id,
                sensor_data
            });
            nbit -= REPORT_SIZE;
        }
        Ok(Self { reports })
    }
}
