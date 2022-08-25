use std;

pub mod cmd;
pub mod stdlib;
pub mod lang;
pub mod vm;

fn main() {
    cmd::init();
    std::process::exit(0);
}
