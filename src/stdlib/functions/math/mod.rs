extern crate log;
use crate::cmd;

pub mod normalize;
pub mod smoothing;

pub fn init_stdlib(cli: &cmd::Cli) {
    normalize::init_stdlib(cli);
    smoothing::init_stdlib(cli);
}
