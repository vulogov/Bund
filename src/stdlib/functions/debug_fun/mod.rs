extern crate log;
use crate::cmd;

pub mod debug_display_hostinfo;

pub fn init_stdlib(cli: &cmd::Cli) {
    debug_display_hostinfo::init_stdlib(cli);
}
