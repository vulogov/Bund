extern crate log;
use crate::cmd;

pub mod banner;

pub fn init_stdlib(cli: &cmd::Cli) {
    banner::init_stdlib(cli);
}
