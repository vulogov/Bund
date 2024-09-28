extern crate log;
use bundcore::bundcore::Bund;
use crate::cmd::Cli;
use crate::cmd::bund_display_banner;

pub fn run(bc: &mut Bund, cli: &Cli) {
    log::debug!("VERSION::run() reached");
    println!("{}", bund_display_banner::bund_banner());
}
