extern crate log;
use crate::cmd;

pub mod make_call_value;
pub mod sort_lists;
pub mod push;
pub mod pull;
pub mod merge;
pub mod unfold;
pub mod getsetinplace;
pub mod listop;

pub fn init_stdlib(cli: &cmd::Cli) {
    make_call_value::init_stdlib(cli);
    sort_lists::init_stdlib(cli);
    push::init_stdlib(cli);
    pull::init_stdlib(cli);
    unfold::init_stdlib(cli);
    getsetinplace::init_stdlib(cli);
    merge::init_stdlib(cli);
    listop::init_stdlib(cli);
}
