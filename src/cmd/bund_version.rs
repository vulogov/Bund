extern crate log;
use bundcore::bundcore::Bund;
use crate::cmd::Cli;
use crate::cmd::bund_display_banner;

pub fn run(_bc: &mut Bund, _cli: &Cli) {
    log::debug!("VERSION::run() reached");
    println!("{}", bund_display_banner::bund_banner());
}
