extern crate log;
use crate::cmd;

pub mod debug_shell;
pub mod debug_display_hostinfo;
pub mod debug_display_distributed_info;
pub mod debug_display_memstats;
pub mod debug_display_stack;
pub mod debug_display_workbench;
pub mod debug_debug;
pub mod debug_dump;
pub mod debug_trace;

pub fn init_stdlib(cli: &cmd::Cli) {
    debug_shell::init_stdlib(cli);
    debug_display_hostinfo::init_stdlib(cli);
    debug_display_distributed_info::init_stdlib(cli);
    debug_display_memstats::init_stdlib(cli);
    debug_display_stack::init_stdlib(cli);
    debug_display_workbench::init_stdlib(cli);
    debug_debug::init_stdlib(cli);
    debug_dump::init_stdlib(cli);
    debug_trace::init_stdlib(cli);
}
