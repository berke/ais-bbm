use ais::{
    AisFragments,
    AisParser,
    sentence::AisSentence,
    messages::AisMessage,
};
use anyhow::{
    Result,
};
use std::{
    fs::{
        File
    },
    ffi::{
        OsString
    },
    io::{
        BufRead,
        BufReader,
    }
};
use pico_args::Arguments;

use ais_bbm::{
    bits::Bits,
    AisBbm
};

fn main()->Result<()> {
    let mut args = Arguments::from_env();
    let verbose = args.contains("--verbose");
    let show_bits = args.contains("--show-bits");
    let inputs : Vec<OsString> = args.finish();
    let mut ais = AisParser::new();
    let mut u = String::new();

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
