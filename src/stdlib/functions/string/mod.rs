extern crate log;
use crate::cmd;

pub mod wildmatch;
pub mod fuzzy_match;
pub mod distance;
pub mod regex;
pub mod regex_matches;
pub mod regex_split;
pub mod prefix_suffix;
pub mod any_id;

pub fn init_stdlib(cli: &cmd::Cli) {
    wildmatch::init_stdlib(cli);
    fuzzy_match::init_stdlib(cli);
    distance::init_stdlib(cli);
    regex::init_stdlib(cli);
    regex_matches::init_stdlib(cli);
    regex_split::init_stdlib(cli);
    prefix_suffix::init_stdlib(cli);
    any_id::init_stdlib(cli);
}
