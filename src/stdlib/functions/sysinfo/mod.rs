extern crate log;
use crate::cmd;

pub mod host;

pub fn init_stdlib(cli: &cmd::Cli) {
    host::init_stdlib(cli);
}
