extern crate log;
use crate::cmd;

pub mod proctitle;
pub mod shell;

pub fn init_stdlib(cli: &cmd::Cli) {
    proctitle::init_stdlib(cli);
    shell::init_stdlib(cli);
}
