extern crate log;
use crate::cmd;

pub mod ai;
pub mod generators;
pub mod bund;
pub mod io;
pub mod bus;
pub mod filesystem;
pub mod debug_fun;
pub mod sysinfo;
pub mod string;
pub mod encoding;
pub mod statistics;
pub mod forecast;
pub mod math;
pub mod system;

pub mod create_aliases;

pub fn init_stdlib(cli: &cmd::Cli) {
    ai::init_stdlib(cli);
    generators::init_stdlib(cli);
    bund::init_stdlib(cli);
    io::init_stdlib(cli);
    bus::init_stdlib(cli);
    encoding::init_stdlib(cli);
    statistics::init_stdlib(cli);
    filesystem::init_stdlib(cli);
    debug_fun::init_stdlib(cli);
    sysinfo::init_stdlib(cli);
    string::init_stdlib(cli);
    system::init_stdlib(cli);
    forecast::init_stdlib(cli);
    math::init_stdlib(cli);
    // And create aliases from BUND shell
    create_aliases::init_stdlib(cli);
}
