extern crate log;
use crate::cmd;

pub mod wildmatch;
pub mod fuzzy_match;
pub mod distance;
pub mod regex;
pub mod regex_matches;
pub mod textexpr_match;
pub mod regex_split;
pub mod prefix_suffix;
pub mod any_id;
pub mod tokenize;
pub mod grok;
pub mod random;
pub mod unicode;
pub mod textwrap;

pub fn init_stdlib(cli: &cmd::Cli) {
    wildmatch::init_stdlib(cli);
    fuzzy_match::init_stdlib(cli);
    distance::init_stdlib(cli);
    regex::init_stdlib(cli);
    regex_matches::init_stdlib(cli);
    regex_split::init_stdlib(cli);
    prefix_suffix::init_stdlib(cli);
    any_id::init_stdlib(cli);
    tokenize::init_stdlib(cli);
    grok::init_stdlib(cli);
    random::init_stdlib(cli);
    unicode::init_stdlib(cli);
    textexpr_match::init_stdlib(cli);
    textwrap::init_stdlib(cli);
}
