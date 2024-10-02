extern crate log;
use crate::cmd;

pub mod load_lines;

pub fn init_stdlib(cli: &cmd::Cli) {
    load_lines::init_stdlib(cli);
}
