extern crate log;
use crate::cmd;

pub mod bund;
pub mod input;
pub mod filesystem;

pub fn init_stdlib(cli: &cmd::Cli) {
    bund::init_stdlib(cli);
    input::init_stdlib(cli);
    filesystem::init_stdlib(cli);
}
