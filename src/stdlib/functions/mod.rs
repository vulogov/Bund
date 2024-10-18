extern crate log;
use crate::cmd;

pub mod bund;
pub mod input;
pub mod filesystem;
pub mod debug_fun;
pub mod sysinfo;

pub fn init_stdlib(cli: &cmd::Cli) {
    bund::init_stdlib(cli);
    input::init_stdlib(cli);
    filesystem::init_stdlib(cli);
    debug_fun::init_stdlib(cli);
    sysinfo::init_stdlib(cli);
}
