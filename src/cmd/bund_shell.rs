extern crate log;
use bundcore::bundcore::Bund;
use crate::cmd;

pub fn run(bc: &mut Bund, cli: &cmd::Cli, shell_arg: &cmd::Shell) {
    log::debug!("SHELL::run() reached");
}
