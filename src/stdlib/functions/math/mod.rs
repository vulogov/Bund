extern crate log;
use crate::cmd;

pub mod normalize;
pub mod smoothing;
pub mod math;
pub mod seq;
pub mod rand;

pub fn init_stdlib(cli: &cmd::Cli) {
    normalize::init_stdlib(cli);
    smoothing::init_stdlib(cli);
    math::init_stdlib(cli);
    seq::init_stdlib(cli);
    rand::init_stdlib(cli);
}
