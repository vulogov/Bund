extern crate log;
use crate::cmd;
use crate::stdlib::banner;

pub fn run_version(_c: &cmd::Cli) {
    log::trace!("run_version() reached");
    println!("{}", banner::bund_banner());
}
