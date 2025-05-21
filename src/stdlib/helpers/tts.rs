extern crate log;
use rust_multistackvm::multistackvm::{VM};
use std::thread;
use std::time::Duration;
use rust_dynamic::types::*;
use crate::stdlib::functions;
use tts::*;
use easy_error::{Error, bail};

pub fn tts_init() {
    thread::spawn(|| {
            log::debug!("Entering TTS thread");
            loop {
                thread::sleep(Duration::from_millis(500));
                match functions::bus::bus_is_empty("force_exit".to_string()) {
                    Ok(res) => {
                        if ! res {
                            let signal = match functions::bus::bus_pull("force_exit".to_string()) {
                                Ok(signal) => match signal.cast_bool() {
                                    Ok(signal) => signal,
                                    Err(_) => continue,
                                }
                                Err(_) => continue,
                            };
                            if signal {
                                log::debug!("Termination requested");
                                break;
                            }
                        }
                    },
                    Err(err) => {
                        log::error!("TTS: Error in force_exit bus: {}", err);
                    }
                }
                match functions::bus::bus_is_empty("tts_in".to_string()) {
                    Ok(data) => {
                        if data {
                            continue;
                        }
                        log::debug!("TTS having a data for TTS");
                        match functions::bus::bus_pull("tts_in".to_string()) {
                            Ok(value) => {
                                match value.type_of() {
                                    STRING => {
                                        println!("{:?}", &value);
                                    }
                                    NODATA => continue,
                                    _ => continue,
                                }
                            },
                            Err(_) => continue,
                        }
                    },
                    Err(err) => {
                        log::debug!("TTS IN queue have an error: {}", err);
                    }
                }
            }
            log::debug!("TTS thread finished");
        }
    );
}

#[time_graph::instrument]
pub fn tts_say(vm: &mut VM, msg: String) -> Result<&mut VM, Error>  {
    // let mut a_tts = match TTS.lock() {
    //     Ok(a_tts) => a_tts,
    //     Err(err) => bail!("TTS::SAY can not lock TTS: {}", err),
    // };
    // let mut local_tts: &mut tts::Tts = match &a_tts.tts {
    //     Some(ref mut local_tts) => local_tts,
    //     None => bail!("TTS interface not available"),
    // };
    // let tts_features: Features = local_tts.supported_features();
    // match local_tts.speak(msg, false) {
    //     Ok(_) => {},
    //     Err(err) => bail!("TTS::SAY returns error: {}", err),
    // };
    // if tts_features.is_speaking {
    //     loop {
    //         let speaking: bool = match local_tts.is_speaking() {
    //             Ok(speaking) => speaking,
    //             Err(err) => {
    //                 drop(local_tts);
    //                 bail!("TTS::SAY returns: {}", err)
    //             },
    //         };
    //         if ! speaking {
    //             break;
    //         } else {
    //             thread::sleep(Duration::from_millis(500));
    //         }
    //     }
    // }
    Ok(vm)
}
