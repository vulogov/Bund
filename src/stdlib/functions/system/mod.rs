extern crate log;
use crate::cmd;

pub mod proctitle;
pub mod shell;
pub mod unixpath;
pub mod ip;
pub mod locale;
pub mod display;
pub mod sleep;

pub fn init_stdlib(cli: &cmd::Cli) {
    proctitle::init_stdlib(cli);
    shell::init_stdlib(cli);
    unixpath::init_stdlib(cli);
    ip::init_stdlib(cli);
    locale::init_stdlib(cli);
    display::init_stdlib(cli);
    sleep::init_stdlib(cli);
}
