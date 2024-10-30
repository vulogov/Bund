extern crate log;
use crate::cmd;

pub mod bund_eval;
pub mod bund_load;
pub mod bund_save;

pub fn init_stdlib(cli: &cmd::Cli) {
    log::debug!("Initializing BUND: module");
    bund_eval::init_stdlib(cli);
    bund_load::init_stdlib(cli);
    bund_save::init_stdlib(cli);
}
