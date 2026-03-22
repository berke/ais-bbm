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

use ais_bbm::AisBbm;

fn main()->Result<()> {
    let mut args = Arguments::from_env();
    let verbose = args.contains("--verbose");
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
                                    match AisBbm::parse(msg)? {
                                        AisBbm::Environmental(env) => {
                                            println!("ENV {:#?}",env);
                                        },
                                        AisBbm::MeteoHydro(mhy) => {
                                            println!("MHY {:#?}",mhy);
                                        },
                                        AisBbm::Unhandled => {
                                            if verbose {
                                                eprintln!("UNHANDLED {:?}",msg);
                                            }
                                        }
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
