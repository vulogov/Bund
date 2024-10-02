extern crate log;
use crate::cmd;

pub mod bund_eval;

pub fn init_stdlib(cli: &cmd::Cli) {
    log::debug!("Initializing BUND: module");
    // bund_eval::init_stdlib(cli);
}
