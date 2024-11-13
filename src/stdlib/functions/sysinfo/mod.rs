extern crate log;
use crate::cmd;

pub mod host;
pub mod virt;
pub mod mem;

pub fn init_stdlib(cli: &cmd::Cli) {
    host::init_stdlib(cli);
    virt::init_stdlib(cli);
    mem::init_stdlib(cli);
}
