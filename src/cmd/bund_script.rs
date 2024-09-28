extern crate log;
use bundcore::bundcore::Bund;
use crate::cmd;

pub fn run(bc: &mut Bund, cli: &cmd::Cli, script_arg: &cmd::Script) {
    log::debug!("SCRIPT::run() reached");
}
