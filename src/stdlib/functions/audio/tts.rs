extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use crate::cmd;
use crate::stdlib::helpers;
use tts::*;
use easy_error::{Error};


pub fn init_stdlib(cli: &cmd::Cli) {
    if Tts::screen_reader_available() {
        log::debug!("TTS: A screen reader is available on this platform.");
    } else {
        log::debug!("TTS: No screen reader is available on this platform.");
    }
    let mut bc = match BUND.lock() {
        Ok(bc) => bc,
        Err(err) => {
            helpers::print_error::print_error_from_str(format!("{}", err), cli);
            return;
        }
    };
    // let _ = bc.vm.register_inline("id.uuid".to_string(), stdlib_uuid_inline);
    drop(bc);
}
