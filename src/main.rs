use std;

pub mod cmd;
pub mod stdlib;
pub mod lang;
pub mod vm;
pub mod twostack;

fn main() {
    cmd::init();
    std::process::exit(0);
}
