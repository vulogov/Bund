extern crate log;
use crate::cmd;

pub mod spinner;
pub mod terminal;

pub fn init_stdlib(cli: &cmd::Cli) {
    spinner::init_stdlib(cli);
    terminal::init_stdlib(cli);
}
