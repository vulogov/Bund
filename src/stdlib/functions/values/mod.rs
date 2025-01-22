extern crate log;
use crate::cmd;

pub mod make_call_value;
pub mod sort_lists;

pub fn init_stdlib(cli: &cmd::Cli) {
    make_call_value::init_stdlib(cli);
    sort_lists::init_stdlib(cli);
}
