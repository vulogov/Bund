extern crate log;
use crate::cmd;

pub mod debug_shell;
pub mod debug_display_hostinfo;
pub mod debug_display_memstats;


pub fn init_stdlib(cli: &cmd::Cli) {
    debug_shell::init_stdlib(cli);
    debug_display_hostinfo::init_stdlib(cli);
    debug_display_memstats::init_stdlib(cli);
}
