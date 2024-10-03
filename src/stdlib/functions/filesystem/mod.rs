extern crate log;
use crate::cmd;

pub mod filepath;

pub fn init_stdlib(cli: &cmd::Cli) {
    filepath::init_stdlib(cli);
}
