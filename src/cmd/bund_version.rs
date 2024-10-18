extern crate log;
use crate::cmd::Cli;
use crate::cmd::bund_display_banner;
use crate::stdlib::functions::debug_fun;

pub fn run(cli: &Cli) {
    log::debug!("VERSION::run() reached");
    println!("{}", bund_display_banner::bund_banner());
    if cli.nocolor {
        debug_fun::debug_display_hostinfo::debug_display_hostinfo_nocolor()
    } else {
        debug_fun::debug_display_hostinfo::debug_display_hostinfo_color()
    }
}
