extern crate log;
use crate::cmd;

pub mod tts;

pub fn init_stdlib(cli: &cmd::Cli) {
    tts::init_stdlib(cli)
}
