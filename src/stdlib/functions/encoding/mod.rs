extern crate log;
use crate::cmd;

pub mod base64;

pub fn init_stdlib(cli: &cmd::Cli) {
    base64::init_stdlib(cli);
}
