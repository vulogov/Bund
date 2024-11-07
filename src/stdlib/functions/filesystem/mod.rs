extern crate log;
use crate::cmd;

pub mod filepath;
pub mod file;
pub mod cp;

pub fn init_stdlib(cli: &cmd::Cli) {
    filepath::init_stdlib(cli);
    file::init_stdlib(cli);
    cp::init_stdlib(cli);
}
