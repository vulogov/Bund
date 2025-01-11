extern crate log;
use crate::cmd;

pub mod html;

pub fn init_stdlib(cli: &cmd::Cli) {
    html::init_stdlib(cli);
}
