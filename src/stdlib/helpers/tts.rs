extern crate log;
use rust_multistackvm::multistackvm::{VM};
use std::thread;
use rust_dynamic::value::Value;
use std::time::Duration;
use rust_dynamic::types::*;
use crate::stdlib::functions;
use tts::*;
use easy_error::{Error, bail};

pub fn tts_speak(msg: String, local_tts: &mut Tts) -> Result<(), Error> {
    log::debug!("Received {} bytes for TTS", &msg.len());

    let tts_features: Features = local_tts.supported_features();
    match local_tts.speak(msg, false) {
        Ok(_) => {
           #[cfg(target_os = "macos")]
           {
               let run_loop = unsafe { objc2_foundation::NSRunLoop::currentRunLoop() };
               unsafe { run_loop.run() };
           }
        },
        Err(err) => bail!("TTS::SAY returns error: {}", err),
    };
    if tts_features.is_speaking {
        loop {
            let speaking: bool = match local_tts.is_speaking() {
                Ok(speaking) => speaking,
                Err(err) => {
                    bail!("TTS::SAY returns: {}", err)
                },
            };
            if ! speaking {
                break;
            } else {
                thread::sleep(Duration::from_millis(500));
            }
        }
    }
    Ok(())
}

pub fn tts_init() {
    thread::spawn(|| {
            log::debug!("Entering TTS thread");
            let mut local_tts = match Tts::default() {
                Ok(local_tts) => local_tts,
                Err(err) => {
                    log::error!("TTS thread initialization had failed: {}", err);
                    return;
                }
            };

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
                                        let msg = match value.cast_string() {
                                            Ok(msg) => msg,
                                            Err(err) => {
                                                log::error!("TTS error casting message: {}", err);
                                                continue;
                                            }
                                        };
                                        match tts_speak(msg, &mut local_tts) {
                                            Ok(_) => {},
                                            Err(err) => {
                                                log::error!("TTS returns error: {}", err);
                                                continue;
                                            }
                                        };
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
    if ! match functions::bus::bus_push("tts_in".to_string(), Value::from_string(msg)) {
        Ok(res) => res,
        Err(err) => bail!("TTS.SAY werror pushing message: {}", err),
    } {
        bail!("TTS.SAY error pushing message");
    }
    Ok(vm)
}
