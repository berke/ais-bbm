use super::*;

// Ref: https://www.e-navigation.nl/content/meteorological-and-hydrographic-data
// DAC 1
// FID 31

#[derive(Clone,Debug)]
pub struct MeteoHydro {
    pub longitude:f64,
    pub latitude:f64,
    pub position_accuracy_high:bool,
    pub day:u8,
    pub hour:u8,
    pub minute:u8,
    pub average_wind_speed:f64,
    pub wind_gust:f64,
    pub wind_direction:f64,
    pub wind_gust_direction:f64,
    pub air_temp:f64,
    pub rel_hum:f64,
    pub dew_point:f64,
    pub air_pressure:f64,
    pub air_pressure_tendency:Tendency,
    pub horizontal_visibility:f64,
    pub water_level:f64,
    pub water_level_trend:Tendency,
    pub surface_current_speed:f64,
    pub surface_current_direction:f64,
    pub current_speed_2:f64,
    pub current_direction_2:f64,
    pub current_meas_level_2:f64,
    pub current_speed_3:f64,
    pub current_direction_3:f64,
    pub current_meas_level_3:f64,
    pub signif_wave_height:f64,
    pub wave_period:f64,
    pub wave_direction:f64,
    pub swell_height:f64,
    pub swell_period:f64,
    pub swell_direction:f64,
    pub sea_state:u8,
    pub water_temp:f64,
    pub precip_type:PrecipitationType,
    pub salinity:f64,
    pub ice:Presence,
    pub spare:u16,
}

#[derive(Clone,Debug)]
pub enum Tendency {
    Steady,
    Decreasing,
    Increasing,
    NotAvailable
}

impl TryFrom<u8> for Tendency {
    type Error = Error;

    fn try_from(x:u8)->Result<Self> {
        Ok(match x {
            0 => Self::Steady,
            1 => Self::Decreasing,
            2 => Self::Increasing,
            3 => Self::NotAvailable,
            n => bail!("Invalid tendency value {}",n)
        })
    }
}

#[derive(Clone,Debug)]
pub enum PrecipitationType {
    Reserved1,
    Rain,
    Thunderstorm,
    FreezingRain,
    MixedIce,
    Snow,
    Reserved2,
    NotAvailable
}

impl TryFrom<u8> for PrecipitationType {
    type Error = Error;

    fn try_from(x:u8)->Result<Self> {
        Ok(match x {
            0 => Self::Reserved1,
            1 => Self::Rain,
            2 => Self::Thunderstorm,
            3 => Self::FreezingRain,
            4 => Self::MixedIce,
            5 => Self::Snow,
            6 => Self::Reserved2,
            _ => Self::NotAvailable
        })
    }
}

#[derive(Clone,Debug)]
pub enum Presence {
    No,
    Yes,
    Reserved,
    NotAvailable
}

impl TryFrom<u8> for Presence {
    type Error = Error;

    fn try_from(x:u8)->Result<Self> {
        Ok(match x {
            0 => Self::No,
            1 => Self::Yes,
            2 => Self::Reserved,
            _ => Self::NotAvailable
        })
    }
}

impl MeteoHydro {
    pub fn parse(msg:&BinaryBroadcastMessage)->Result<Self> {
        let mut u = Bits::from(&msg.data[..]);
	let longitude = to_ll(25,1e3,u.bits::<u32>(25)?);
	let latitude = to_ll(24,1e3,u.bits::<u32>(24)?);
	let position_accuracy_high = u.bit()?;
	let day = u.bits::<u8>(5)?;
	let hour = u.bits::<u8>(5)?;
	let minute = u.bits::<u8>(6)?;
	let average_wind_speed = to_range(1.0,0,126,u.bits::<u32>(7)?);
	let wind_gust = to_range(1.0,0,126,u.bits::<u32>(7)?);
	let wind_direction = to_range(1.0,0,359,u.bits::<u32>(9)?);
	let wind_gust_direction = to_range(1.0,0,359,u.bits::<u32>(9)?);
	let air_temp = to_range_signed(11,0.1,-600,600,u.bits::<u32>(11)?);
	let rel_hum = to_range(1.0,0,100,u.bits::<u32>(7)?);
	let dew_point = to_range_signed(10,1.0,-200,500,u.bits::<u32>(10)?);
	let air_pressure = 799.0 + to_range(1.0,1,401,u.bits::<u32>(9)?);
	let air_pressure_tendency = u.bits::<u8>(2)?.try_into()?;
	let horizontal_visibility = to_range(0.1,0,126,u.bits::<u32>(8)?);
	let water_level = -10.0 + to_range(0.01,0,4000,u.bits::<u32>(12)?);
	let water_level_trend = u.bits::<u8>(2)?.try_into()?;
	let surface_current_speed = to_range(0.1,0,250,u.bits::<u32>(8)?);
	let surface_current_direction = to_range(1.0,0,359,u.bits::<u32>(9)?);
	let current_speed_2 = to_range(0.1,0,250,u.bits::<u32>(8)?);
	let current_direction_2 = to_range(1.0,0,359,u.bits::<u32>(9)?);
	let current_meas_level_2 = to_range(1.0,0,30,u.bits::<u32>(5)?);
	let current_speed_3 = to_range(0.1,0,250,u.bits::<u32>(8)?);
	let current_direction_3 = to_range(1.0,0,359,u.bits::<u32>(9)?);
	let current_meas_level_3 = to_range(1.0,0,30,u.bits::<u32>(5)?);
	let signif_wave_height = to_range(0.1,0,250,u.bits::<u32>(8)?);
	let wave_period = to_range(1.0,0,60,u.bits::<u32>(6)?);
	let wave_direction = to_range(1.0,0,359,u.bits::<u32>(9)?);
	let swell_height = to_range(0.1,0,250,u.bits::<u32>(8)?);
	let swell_period = to_range(1.0,0,60,u.bits::<u32>(6)?);
	let swell_direction = to_range(1.0,0,359,u.bits::<u32>(9)?);
	let sea_state = u.bits::<u8>(4)?;
	let water_temp = to_range_signed(10,0.1,-100,500,u.bits::<u32>(10)?);
	let precip_type = u.bits::<u8>(3)?.try_into()?;
	let salinity = to_range(0.1,0,500,u.bits::<u32>(9)?);
	let ice = u.bits::<u8>(2)?.try_into()?;
	let spare = u.bits::<u16>(10)?;
	assert!(u.is_empty());
	Ok(Self {
	    longitude,
	    latitude,
	    position_accuracy_high,
	    day,
	    hour,
	    minute,
	    average_wind_speed,
	    wind_gust,
	    wind_direction,
	    wind_gust_direction,
	    air_temp,
	    rel_hum,
	    dew_point,
	    air_pressure,
	    air_pressure_tendency,
	    horizontal_visibility,
	    water_level,
	    water_level_trend,
	    surface_current_speed,
	    surface_current_direction,
	    current_speed_2,
	    current_direction_2,
	    current_meas_level_2,
	    current_speed_3,
	    current_direction_3,
	    current_meas_level_3,
	    signif_wave_height,
	    wave_direction,
	    wave_period,
	    swell_height,
	    swell_direction,
	    swell_period,
	    sea_state,
	    water_temp,
	    precip_type,
	    salinity,
	    ice,
	    spare
	})
    }
}
