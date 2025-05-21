extern crate log;
use crate::stdlib::BUND;
use rust_multistackvm::multistackvm::{VM};
use crate::stdlib::helpers;
use crate::cmd;
use rust_dynamic::types::*;
use easy_error::{Error, bail};
use tts::Tts;

#[time_graph::instrument]
pub fn stdlib_tts_speak_inline(vm: &mut VM) -> Result<&mut VM, Error> {
    let value = match vm.stack.pull() {
        Some(value) => value,
        None => bail!("TTS.ASK: NO DATA #1"),
    };
    let msg = match value.type_of() {
        STRING => {
            let msg = match value.cast_string() {
                Ok(msg) => msg,
                Err(err) => bail!("TTS.SAY error casting message: {}", err),
            };
            msg
        },
        OBJECT => bail!("Not supported yet"),
        _ =>  bail!("TTS.SAY get's an object of unsupported type from the stack"),
    };
    let res = helpers::tts::tts_say(vm, msg);
    res
}

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
    let _ = bc.vm.register_inline("tts.say".to_string(), stdlib_tts_speak_inline);
    drop(bc);
}
