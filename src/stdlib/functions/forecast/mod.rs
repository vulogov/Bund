extern crate log;
use crate::cmd;

pub mod markov;

pub fn init_stdlib(cli: &cmd::Cli) {
    markov::init_stdlib(cli);
}
