extern crate log;
use crate::cmd;

#[derive(Debug, Clone)]
pub enum SourceMode {
    Consume,
    Keep,
}

pub mod get_data;

pub mod count;
pub mod statistics;

pub fn init_stdlib(cli: &cmd::Cli) {
    count::init_stdlib(cli);
    statistics::init_stdlib(cli);
}
