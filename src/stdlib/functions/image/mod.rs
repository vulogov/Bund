extern crate log;
use crate::cmd;

pub mod print_image;


pub fn init_stdlib(cli: &cmd::Cli) {
    print_image::init_stdlib(cli);
}
