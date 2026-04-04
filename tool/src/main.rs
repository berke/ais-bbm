use ais::{
    AisFragments,
    AisParser,
    sentence::AisSentence,
    messages::AisMessage,
};
use anyhow::{
    bail,
    Result,
};
use std::{
    collections::HashSet,
    fs::{
        File
    },
    ffi::{
        OsString
    },
    io::{
        BufRead,
        BufReader,
    },
    str::{
        FromStr
    }
};
use pico_args::Arguments;

use ais_bbm::{
    bits::Bits,
    AisBbm
};

fn pair_from_str<T>(u:&str)->Result<(T,T)>
where
    T:FromStr,
    <T as FromStr>::Err : std::error::Error + Send + Sync + 'static
{
    if let Some((x,y)) = u.split_once(',') {
	let x = T::from_str(x)?;
	let y = T::from_str(y)?;
	return Ok((x,y))
    }
    bail!("Invalid pair {:?}",u);
}

fn main()->Result<()> {
    let mut args = Arguments::from_env();
    let verbose = args.contains("--verbose");
    let show_bits = args.contains("--show-bits");
    let mmsis : Vec<u32> = args.values_from_str("--mmsi")?;
    let coords : Vec<(f64,f64)> = args.values_from_fn("--coords",pair_from_str)?;
    let coord_tol : f64 = args.opt_value_from_str("--coord-tol")?
        .unwrap_or(1e-4);
    let inputs : Vec<OsString> = args.finish();
    let mut ais = AisParser::new();
    let mut u = String::new();

    let mmsi_set : HashSet<u32> = mmsis.iter().cloned().collect();
    if !mmsi_set.is_empty() {
        for mmsi in &mmsi_set {
            println!("  {}",mmsi);
        }
    }

    for input in inputs {
        let fd = File::open(&input)?;
        let mut br = BufReader::new(fd);
        loop {
            u.clear();
            let m = br.read_line(&mut u)?;
            if m == 0 {
                break;
            }
            match ais.parse(u.as_bytes(),true) {
                Ok(fr) => {
                    if verbose {
                        println!("{}\n{:#?}",u.trim(),fr);
                    }
                    match fr {
                        AisFragments::Complete(
                            AisSentence {
                                message:Some(ref msg),
                                ..
                            }) =>
                        {
                            match msg {
                                AisMessage::BinaryBroadcastMessage(msg) => {
                                    if !mmsi_set.is_empty() &&
                                        !mmsi_set.contains(&msg.mmsi)
                                    {
                                        continue;
                                    }

                                    print!("BBM {} MMSI:{} DAC:{} FID:{} ",
                                           msg.message_type,
                                           msg.mmsi,
                                           msg.dac,
                                           msg.fid);
                                    if show_bits {
                                        println!();
                                        let mut u = Bits::from(&msg.data[..]);
                                        let mut n = u.len();
                                        let mut i = 0;
                                        while n > 0 {
                                            let p = n.min(32);
                                            let x = u.bits::<u32>(p as u32)?;
                                            println!("{i:04} {x:00$b}",p);
                                            n -= p;
                                            i += p;
                                        }
                                    }
                                    match AisBbm::parse(msg) {
                                        Ok(bbm) => {
                                            match bbm {
                                                AisBbm::Environmental(env) => {
                                                    println!("ENV {:#?}",env);
                                                },
                                                AisBbm::MeteoHydro(mhy) => {
                                                    let mut found = false;
                                                    if !coords.is_empty() {
                                                        let xm = mhy.longitude;
                                                        let ym = mhy.latitude;
                                                        for &(x,y) in &coords {
                                                            let e = (x - xm).abs() + (y - ym).abs();
                                                            if e < coord_tol {
                                                                found = true;
                                                                break;
                                                            }
                                                        }
                                                        if !found {
                                                            continue;
                                                        }
                                                    }
                                                    println!("MHY {:#?}",mhy);
                                                },
                                                AisBbm::InlandShipVoyage(isv) => {
                                                    println!("ISV {:#?}",isv);
                                                },
                                                AisBbm::Unhandled => {
                                                    if verbose {
                                                        eprintln!("UNHANDLED {:?}",msg);
                                                    }
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("Error parsing BBM: {}",e);
                                        },
                                    }
                                },
                                _ => ()
                            }
                        },
                        _ => ()
                    }
                },
                Err(e) => {
                    if verbose {
                        eprintln!("ERR {}",e);
                    }
                }
            }
        }
    }
        
    Ok(())
}
