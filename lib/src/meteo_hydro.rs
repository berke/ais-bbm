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
    pub air_pressure_tendency:u8,
    pub horizontal_visibility:f64,
    pub water_level:f64,
    pub water_level_trend:u8,
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
    pub precip_type:u8,
    pub salinity:f64,
    pub ice:u8,
    pub spare:u16,
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
	let air_pressure_tendency = u.bits::<u8>(2)?;
	let horizontal_visibility = to_range(0.1,0,126,u.bits::<u32>(8)?);
	let water_level = -10.0 + to_range(0.01,0,4000,u.bits::<u32>(12)?);
	let water_level_trend = u.bits::<u8>(2)?;
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
	let precip_type = u.bits::<u8>(3)?;
	let salinity = to_range(0.1,0,500,u.bits::<u32>(9)?);
	let ice = u.bits::<u8>(2)?;
	let spare = u.bits::<u16>(10)?;
	// assert!(u.is_empty());
	while !u.is_empty() {
	    let n = u.len().min(8);
	    let x = u.bits::<u8>(n as u32)?;
	    println!("{x:00$b}",n);
	}
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
