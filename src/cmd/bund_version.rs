extern crate log;
use crate::cmd::Cli;
use crate::cmd::bund_display_banner;

pub fn run(_cli: &Cli) {
    log::debug!("VERSION::run() reached");
    println!("{}", bund_display_banner::bund_banner());
}
