extern crate log;

pub mod load_lines;

pub fn init_stdlib() {
    load_lines::init_stdlib();
}
