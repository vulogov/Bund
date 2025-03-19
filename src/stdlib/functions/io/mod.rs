extern crate log;
use crate::cmd;

pub mod banner;
pub mod textfile;
pub mod graph;
pub mod input;
pub mod terminal;

pub fn init_stdlib(cli: &cmd::Cli) {
    banner::init_stdlib(cli);
    textfile::init_stdlib(cli);
    graph::init_stdlib(cli);
    input::init_stdlib(cli);
    terminal::init_stdlib(cli);
}
