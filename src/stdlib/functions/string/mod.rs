extern crate log;
use crate::cmd;

pub mod wildmatch;
pub mod regex;

pub fn init_stdlib(cli: &cmd::Cli) {
    wildmatch::init_stdlib(cli);
    regex::init_stdlib(cli);
}
