extern crate log;
use crate::cmd;

pub mod wildmatch;
pub mod regex;
pub mod regex_matches;
pub mod regex_split;

pub fn init_stdlib(cli: &cmd::Cli) {
    wildmatch::init_stdlib(cli);
    regex::init_stdlib(cli);
    regex_matches::init_stdlib(cli);
    regex_split::init_stdlib(cli);
}