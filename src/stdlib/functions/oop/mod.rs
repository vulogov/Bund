extern crate log;
use crate::cmd;

pub mod base_classes;
pub mod value_class;

pub fn init_stdlib(cli: &cmd::Cli) {
    base_classes::init_stdlib(cli);
    value_class::init_stdlib(cli);
}
