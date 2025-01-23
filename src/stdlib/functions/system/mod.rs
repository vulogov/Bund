extern crate log;
use crate::cmd;

pub mod proctitle;
pub mod shell;
pub mod unixpath;
pub mod ip;

pub fn init_stdlib(cli: &cmd::Cli) {
    proctitle::init_stdlib(cli);
    shell::init_stdlib(cli);
    unixpath::init_stdlib(cli);
    ip::init_stdlib(cli);
}
