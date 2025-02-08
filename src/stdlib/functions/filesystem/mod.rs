extern crate log;
use crate::cmd;

pub mod filepath;
pub mod filesystem;
pub mod file;
pub mod cp;
pub mod ls;
pub mod cwd;
pub mod file_write;

pub fn init_stdlib(cli: &cmd::Cli) {
    filepath::init_stdlib(cli);
    filesystem::init_stdlib(cli);
    file::init_stdlib(cli);
    cp::init_stdlib(cli);
    ls::init_stdlib(cli);
    cwd::init_stdlib(cli);
    file_write::init_stdlib(cli);
}
